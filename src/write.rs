use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Write};
use std::sync::mpsc::Receiver;

pub fn write_loop(outfile: &str, writer_rx: Receiver<Vec<u8>>) -> io::Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    while let Ok(buffer) = writer_rx.recv() {
        if buffer.is_empty() {
            break;
        }
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}
