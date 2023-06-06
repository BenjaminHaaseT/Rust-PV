use crossbeam::channel::{bounded, unbounded};
use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let Args {
        infile,
        outfile,
        silent,
    } = Args::parse();
    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    // let quit = Arc::new(Mutex::new(false));
    // let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_handle = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_handle?;

    Ok(())
}
