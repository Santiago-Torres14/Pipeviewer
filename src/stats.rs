use std::io;
use std::sync::mpsc::Receiver;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> io::Result<()>{
    let mut total_bytes = 0;

    while let Ok(num_read) = stats_rx.recv() {
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if num_read == 0 {
            break;
        }
    }

    if !silent {
        eprintln!("");
    }
    Ok(())
}