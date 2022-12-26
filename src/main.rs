mod sim;

use clap::Parser;
use std::io;
use std::path::PathBuf;
use log::{debug, error, info, warn};
use mpi::request::WaitGuard;
use mpi::traits::*;
use crate::sim::proc::{get_slice, Proc};

use crate::sim::star::{from_csv, Particle};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}


fn main() -> io::Result<()> {
    env_logger::init();

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    if rank == 0 {
        info!("Starting with [size={}]", size)
    }

    let args: Args = Args::parse();
    let file_buff = args.file
        .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;
    if rank == 0 {
        info!("Reading from [file={:?}]", file_buff)
    }
    let stars = from_csv(file_buff.to_str().unwrap())?;

    // debug!("rank={}, {:?}", rank, get_slice(size, rank, stars.len()));
    let mut proc = Proc::new(&world,
                             stars[get_slice(size, rank, stars.len())].to_vec(),
                             rank,
                             size);
    // warn!("{}; proc; {:?} {:?}", {rank}, proc.stars, proc.stars_buff);

    world.barrier();
    for it in 0..5 {
        for _ in 0..size {
            match proc.step() {
                Ok(()) => {}
                Err(err) => {
                    error!("Error: {}", err);
                    break
                }
            }
            world.barrier();
        }
        world.barrier();
        info!("[i{}][r{}] State: {:?}", it, rank, proc.stars)
    }

    Ok(())
}