use std::fs::File;
use std::{fmt, io, ops};
use std::error::Error;
use std::io::{BufRead, BufReader};
use log::{debug, error, info, trace, warn};
use mpi::traits::Equivalence;
use crate::sim::err::StarErr;

type Unit = f32;

#[derive(Copy, Clone, Equivalence, Debug, PartialEq)]
pub struct Vec3(Unit, Unit, Unit);

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Vec3 {
    const EPS_SQ: Unit = 0f32;

    pub fn dist3d(self, other: Self) -> Unit {
        (self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + Vec3::EPS_SQ).sqrt()
    }

    pub fn from_vec(vec: Vec<&str>) -> io::Result<Self> {
        if vec.len() != 3 {
            error!("Cannot parse vec3");
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Too few arguments for ve3"))
        }
        Ok(Self {
            0: vec.get(0).unwrap().parse::<Unit>()
                .map_err(|_| Particle::cannot_parse())?,
            1: vec.get(1).unwrap().parse::<Unit>()
                .map_err(|_| Particle::cannot_parse())?,
            2: vec.get(2).unwrap().parse::<Unit>()
                .map_err(|_| Particle::cannot_parse())?,
        })
    }

    pub fn zeros() -> Self {
        Vec3(0f32, 0f32, 0f32)
    }
}


#[derive(Copy, Clone, Equivalence, PartialEq)]
pub struct Particle {
    pos: Vec3,
    vel: Vec3,

    mass: Unit, radius: Unit,

    new_vel: Vec3
}

impl Particle {
    const G: Unit = 10f32;
    const DT: Unit = 0.1f32;

    pub fn calc(&mut self, other: &Self) {
        todo!()
    }

    pub fn calc_vec(&mut self, other: &Vec<Self>) {
        trace!("[s{}] ?? {:?}", self.mass, other);
        let mut acc = Vec3(0f32, 0f32, 0f32);
        let mut new_vel = Vec3(0f32, 0f32, 0f32);


        other.into_iter().filter(|other| **other != *self).for_each(|other| {
            let mut dpos = other.pos - self.pos;
            let r3 = self.dist3d(other).powf(3f32);

            acc.0 += Particle::G * other.mass * dpos.0 / r3;
            acc.1 += Particle::G * other.mass * dpos.1 / r3;
            acc.2 += Particle::G * other.mass * dpos.2 / r3;

            trace!("[s{}] !! {}", self.mass, other.mass);
        });

        self.new_vel.0 += acc.0 * Particle::DT;
        self.new_vel.1 += acc.1 * Particle::DT;
        self.new_vel.2 += acc.2 * Particle::DT;
    }

    pub fn new() -> Self{
        Particle {
            pos: Vec3(1.01f32,1f32,1f32),
            vel: Vec3(1.01f32, 1f32, 1f32),
            mass: 5.0,
            radius: 1.0,
            new_vel: Vec3(0f32, 0f32, 0f32)
        }
    }

    pub fn from_str(string: &str) -> io::Result<Self> {
        let err = io::Error::new(io::ErrorKind::InvalidInput, "Cannot parse input; Too few arguments");

        let res: Vec<&str> = string.split(",").collect();
        if let [pos, vel, rest] = res.chunks(3).collect::<Vec<&[&str]>>().as_slice() {
            let pos = Vec3::from_vec(pos.to_vec())?;
            let vel = Vec3::from_vec(vel.to_vec())?;

            let mass = rest.get(0).ok_or(Particle::too_few_args())?.parse::<Unit>()
                .map_err(|_| Particle::cannot_parse())?;
            let radius = rest.get(1).ok_or(Particle::too_few_args())?.parse::<Unit>()
                .map_err(|_| Particle::cannot_parse())?;

            Ok(Particle {
                pos,
                vel,
                mass,
                radius,
                new_vel: Vec3(0f32, 0f32, 0f32)
            })
        } else {
            Err(err)
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{}", self.pos.0, self.pos.1, self.pos.2)
    }

    pub fn dist3d(&self, other: &Self) -> Unit {
        self.pos.dist3d(other.pos)
    }

    pub fn update(&mut self) {
        self.pos.0 += self.vel.0 * Particle::DT;
        self.pos.1 += self.vel.1 * Particle::DT;
        self.pos.2 += self.vel.2 * Particle::DT;

        self.vel.0 += self.new_vel.0;
        self.vel.1 += self.new_vel.1;
        self.vel.2 += self.new_vel.2;

        self.new_vel = Vec3::zeros();
    }

    pub fn get_mass(&self) -> Unit {
        self.mass
    }

    pub fn get_new_vel(&self) -> Vec3 {
        self.new_vel
    }

    fn cannot_parse() -> io::Error {
        io::Error::new(io::ErrorKind::InvalidInput, "Cannot parse")
    }

    fn too_few_args() -> io::Error {
        io::Error::new(io::ErrorKind::InvalidInput, "Cannot parse input; Too few arguments")
    }
}


impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Star")
            .field("x", &self.pos.0)
            .field("y", &self.pos.1)
            .field("z", &self.pos.2)
            .field("mass", &self.mass)
            .finish()
    }
}

pub fn from_csv(path: &str) -> io::Result<Vec<Particle>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let x: Result<Vec<Particle>, _> = reader.lines()
        .skip(1)
        .map(|line| {
            match line {
                Ok(line) => Particle::from_str(line.as_str()),
                Err(err) => Err(err)
            }
        }).collect();

    info!("Parsed {:?} lines", x.as_ref().unwrap_or(&vec![]).len());

    Ok(x?)
}