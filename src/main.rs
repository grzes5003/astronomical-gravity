mod sim;

use std::fs::{File, OpenOptions};
use std::io::Write;
use clap::Parser;
use std::io;
use std::path::PathBuf;
use log::{debug, error, info, trace, warn};
use mpi::request::{StaticScope, WaitGuard};
use mpi::traits::*;
use crate::sim::proc::{get_slice, Proc};

use crate::sim::star::{from_csv, Particle};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
    #[arg(long = "save", short = 's', default_value_t = false)]
    save_results: bool,
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
    let save_result = args.save_results;
    let mut output = None;


    if save_result && rank == 0 {
        let path = file_buff.parent().unwrap()
            .with_file_name("output")
            .with_extension("csv")
            .to_str().unwrap().to_string();
        output = Some(OpenOptions::new().write(true).create(true).open(path)?);
    }

    if rank == 0 {
        info!("Reading from [file={:?}]", file_buff)
    }
    let stars = from_csv(file_buff.to_str().unwrap())?;

    let mut proc = Proc::new(&world,
                             stars[get_slice(size, rank, stars.len())].to_vec(),
                             rank,
                             size);

    world.barrier();
    let t_start = mpi::time();
    for it in 0..10 {
        for _ in 0..size {
            match proc.step() {
                Ok(()) => {}
                Err(err) => {
                    error!("Error: {}", err);
                    break;
                }
            }
            debug!("[i{}][r{}] next step", it, rank);
            world.barrier();
        }
        world.barrier();
        debug!("[i{}][r{}] Finished iteration", it, rank);
        proc.complete_interation();
        trace!("[i{}][r{}] State: {:?}", it, rank, proc.stars.len());
    }
    world.barrier();
    if rank == 0 {
        let t_end = mpi::time();
        println!("t={};s={};p={};f={}",
                 t_end - t_start,
                 stars.len(),
                 world.size(),
                 file_buff.file_name().unwrap().to_str().unwrap_or("none"));
    }
    if save_result {
        let chunk = proc.stars.iter().map(|star| star.to_string() + ",")
            .collect::<String>()
            .as_bytes()
            .to_vec();
        trace!("[{}] chunk: {:?}", rank, chunk);
        if rank == 0 {
            let mut line = vec![0u8; chunk.len() * (world.size() as usize + 1)];
            world.process_at_rank(0)
                .gather_into_root(&chunk[..], &mut line[..]);
            line = line.into_iter().filter(|ch| *ch != 0u8).collect();
            trace!("[{}] gathered data", rank);
            output.as_ref().unwrap()
                .write_all(
                    format!("{}\n", std::str::from_utf8(line.as_slice()).unwrap()).as_bytes()
                ).unwrap();
        } else {
            world.process_at_rank(0).gather_into(&chunk[..])
        }
    }

    info!("DONE");
    Ok(())
}