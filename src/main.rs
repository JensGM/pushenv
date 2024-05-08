use std::os::unix::process::CommandExt as _;
use clap::Parser as _;

#[derive(clap::Parser, Debug)]
#[command(name = "pushenv")]
#[command(about = "Push environment variables to the shell")]
struct Cli {
    envfile: Option<String>,
    #[clap(last = true)]
    cmd: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
enum PushEnvError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Dotenv(#[from] dotenv::Error),
    #[error("Missing command")]
    MissingCommand,
}

fn main2() -> Result<(), PushEnvError> {
    let args = Cli::parse();
    dotenv::from_filename(args.envfile.unwrap_or(".env".to_string()))?;

    let cmd = match args.cmd.first() {
        Some(cmd) => cmd,
        None => return Err(PushEnvError::MissingCommand),
    };

    let args = &args.cmd[1..];
    let err = std::process::Command::new(cmd).args(args).exec();
    Err(PushEnvError::Io(err))
}

fn main() -> () {
    if let Err(e) = main2() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
