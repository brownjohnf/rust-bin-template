use log::debug;
use std::path::PathBuf;
use structopt::StructOpt;

use env_logger;
use log::info;

#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(rename_all = "kebab-case")]
/// Program description here
struct Cli {
    #[structopt(short, long)]
    /// Path to some file or directory on the system
    path: Option<PathBuf>,

    #[structopt(short, long)]
    /// Optional string value
    option1: Option<String>,

    #[structopt(last = true)]
    /// Description for a list of strings
    option2: Vec<String>,

    #[structopt(long, short = "n")]
    /// Don't take any destructive actions
    dry_run: bool,

    #[structopt(long, short)]
    /// Enable debug logging
    verbose: bool,

    // Register subcommands
    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

// Subcommands
#[derive(StructOpt, Debug)]
enum Cmd {
    #[structopt(name = "list")]
    /// Some sort of list operation
    List {
        /// Semver for this release
        #[structopt(short, long)]
        version: String,
    },

    #[structopt(name = "get")]
    /// Some sort of get operation, with flags
    Get {},
}
fn main() {
    debug!("running");

    let args = Cli::from_args();

    // Figure out what log level to use
    let log_level = match &args.verbose {
        true => "debug",
        _ => "info",
    };

    // Set up the logger to output more nicely formatted logs at the right log level based on
    // whether the verbose flag is set
    let mut builder =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level));
    builder.default_format_timestamp(false).init();

    std::process::exit(match run(args) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

fn run(opt: Cli) -> Result<(), changeme::Error> {
    // Parse some optional string value so we can use it
    let option1 = &opt.option1.unwrap_or("".to_string());

    // Potentially set up the library that this CLI is wrapping
    let changeme = changeme::Changeme::new(option1);

    debug!("debug: {}", &changeme.foo);
    info!("debug: {}", &changeme.foo);

    // Match on whether a subcommand was passed
    match &opt.cmd {
        // If there's a subcommand, match on the subcommand
        Some(cmd) => match cmd {
            Cmd::List { version } => changeme.list(version),
            Cmd::Get {} => changeme.get(),
        },
        // Otherwise, just process flags
        None => {
            info!("no subcommand passed");
            Ok(())
        }
    }
}
