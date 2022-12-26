mod sim;

use std::io;
use log::{debug, error, info, warn};
use mpi::request::WaitGuard;
use mpi::traits::*;
use crate::sim::proc::{get_slice, Proc};

use crate::sim::star::{from_csv, Particle};

fn main() -> io::Result<()> {
    env_logger::init();

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    if rank == 0 {
        info!("Starting with [size={}]", size)
    }

    from_csv("../../result.csv")?;

    let stars = vec![Particle::new(), Particle::new(), Particle::new(), Particle::new()];

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