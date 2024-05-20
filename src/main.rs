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

#[cfg(unix)]
fn exec(cmd: &str, args: &[String]) -> Result<(), PushEnvError> {
    use std::os::unix::process::CommandExt as _;
    let err = std::process::Command::new(cmd).args(args).exec();
    Err(err.into())
}

#[cfg(windows)]
fn exec(cmd: &str, args: &[String]) -> Result<(), PushEnvError> {
    let status = std::process::Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(PushEnvError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command failed with status: {}", status),
        )))
    }
}

fn main2() -> Result<(), PushEnvError> {
    let args = Cli::parse();
    dotenv::from_filename(args.envfile.unwrap_or(".env".to_string()))?;

    let cmd = match args.cmd.first() {
        Some(cmd) => cmd,
        None => return Err(PushEnvError::MissingCommand),
    };

    let args = &args.cmd[1..];
    exec(cmd, args)
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
    fn test_pushenv(#[case] envfile_name: Option<&str>) {
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
