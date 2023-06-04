use clap::{Arg, Command};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> io::Result<()> {
    let mut matches = Command::new("my pipeviewer")
        .arg(
            Arg::new("infile")
                .help("Read from a file instead of stdin")
                .default_value(""),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .require_equals(true)
                .help("Write output to a file instead of stdout"),
        )
        .arg(Arg::new("silent").short('s').long("silent"))
        .get_matches();

    let infile: String = matches.remove_one("infile").unwrap_or_default();
    let outfile: String = matches.remove_one("outfile").unwrap_or_default();
    let silent = if matches.contains_id("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => bytes_read,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = writer.write_all(&mut buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }

    if !silent {
        eprintln!("bytes read: {}", total_bytes);
    }

    Ok(())
}
