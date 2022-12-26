use std::cmp::min;
use std::ops::Range;
use log::{debug, error, trace, warn};
use mpi::point_to_point::Status;
use mpi::Rank;
use mpi::topology::SystemCommunicator;
use mpi::traits::*;
use crate::sim::err::StarErr;
use crate::sim::star::Particle;


pub struct Proc<'a> {
    comm: &'a SystemCommunicator,

    pub stars: Vec<Particle>,
    pub stars_buff: Vec<Particle>,

    next_rank: Rank,
    previous_rank: Rank,

    iteration: usize,
}

impl<'a> Proc<'a> {
    pub fn step(&mut self) -> Result<(), StarErr> {
        debug!("step rank={}", self.previous_rank + 1);

        // inc iteration
        self.iteration += 1;

        if self.stars_buff.len() == 0 {
            return Err(StarErr::from("Empty initial state"));
        }

        // send passing stars
        trace!("{}; sending {:?}", self.previous_rank + 1, self.stars_buff);
        self.comm.process_at_rank(self.next_rank)
            .send(self.stars_buff.as_slice());

        // rcv passing stars
        let (rcv, msg) = self.comm
            .process_at_rank(self.previous_rank)
            .receive_vec();
        trace!("got {:?} {:?}", rcv, msg);

        self.stars.iter_mut()
            .for_each(|star| star.calc_vec(&rcv));

        // set buff
        self.stars_buff = rcv;

        Ok(())
    }

    pub fn new(comm: &'a SystemCommunicator, stars: Vec<Particle>, rank: Rank, size: Rank) -> Self {
        let next_rank = (rank + 1) % size;
        let previous_rank = (rank - 1 + size) % size;

        Proc {
            comm,
            stars: stars.clone(),
            stars_buff: stars,
            next_rank,
            previous_rank,
            iteration: 0,
        }
    }
}

pub fn get_slice(size: Rank, rank: Rank, arr_len: usize) -> Range<usize> {
    let (div, md) = (arr_len / size as usize, arr_len % size as usize);
    (rank as usize * div + min(rank as usize, md))..
        ((rank as usize + 1) * div + min(rank as usize + 1, md))
}