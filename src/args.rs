use clap::{Arg, Command};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
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

        Args {
            infile,
            outfile,
            silent,
        }
    }
}
