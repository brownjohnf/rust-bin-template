use log::debug;
use std::{fmt};
use structopt::StructOpt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Argument { description: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Argument { description } => write!(f, "Argument error: {}", description),
        }
    }
}

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(rename_all = "kebab-case")]
/// Program description here
struct Cli {
    #[structopt(short, long)]
    /// Description of the option
    option1: Option<String>,

    #[structopt(last = true)]
    /// Description for a list of strings
    option2: Vec<String>,
}

fn main() {
    debug!("running");
    env_logger::init();

    let args = Cli::from_args();

    std::process::exit(match run(args) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

fn run(args: Cli) -> Result<(), Error> {
    Ok(())
}
