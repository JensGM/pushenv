use clap::Parser as _;
use std::os::unix::process::CommandExt as _;

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

#[cfg(test)]
mod tests {
    fn create_envfile(path: &std::path::Path) {
        use std::io::Write as _;
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "TEST_VAR_1=foo").unwrap();
        writeln!(file, "TEST_VAR_2=bar").unwrap();
    }

    #[rstest::rstest]
    #[case(None)]
    #[case(Some("some.env.file"))]
    fn test_pushenv_without_explicit_envfile(#[case] envfile_name: Option<&str>) {
        let dir = tempfile::tempdir().unwrap();
        let envfile = dir.path().join(envfile_name.unwrap_or(".env"));
        create_envfile(&envfile);

        let mut cmd = assert_cmd::Command::cargo_bin("pushenv").unwrap();
        cmd.current_dir(dir.path());

        if let Some(e) = envfile_name {
            cmd.arg(e);
        }

        cmd.arg("--").arg("env");
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("TEST_VAR_1=foo"))
            .stdout(predicates::str::contains("TEST_VAR_2=bar"));
    }
}
