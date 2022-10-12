use pipeviewer::{args::Piperviewer, read, stats, write};
use std::io::Result;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let args = Piperviewer::new();
    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();
    let read_thread =
        thread::spawn(move || read::read_loop(args.infile.as_deref().unwrap(), stats_tx, write_tx));
    let stat_thread = thread::spawn(move || stats::stats_loop(args.silent, stats_rx));
    let write_thread =
        thread::spawn(move || write::write_loop(args.outfile.as_deref().unwrap(), write_rx));

    read_thread.join().unwrap()?;
    stat_thread.join().unwrap()?;
    write_thread.join().unwrap()?;

    Ok(())
}
