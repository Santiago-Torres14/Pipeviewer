use std::io;
use std::sync::mpsc::Receiver;
use std::time;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> io::Result<()> {
    let mut total_bytes = 0;
    let start = time::Instant::now();
    let mut timer = Timer::new();
    while let Ok(num_read) = stats_rx.recv() {
        timer.update();
        total_bytes += num_read;
        let rate_per_second = num_read as f64 / timer.delta.as_secs_f64();
        if !silent && timer.ready {
            eprint!("\r{} {} [{:.0}b/s]", total_bytes, start.elapsed().as_secs(), rate_per_second);
        }
        if num_read == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }
    Ok(())
}

struct Timer {
    last_instant: time::Instant,
    delta: time::Duration,
    period: time::Duration,
    countdown: time::Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        Self {
            last_instant: time::Instant::now(),
            delta: time::Duration::default(),
            period: time::Duration::from_millis(1000),
            countdown: time::Duration::default(),
            ready: true,
        }
    }

    fn update(&mut self) {
        let now = time::Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
