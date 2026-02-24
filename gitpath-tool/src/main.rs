use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

#[derive(Parser, Debug)]
#[command(name = "gitpath-tool")]
#[command(about = "Resolve git path from --git-path, GIT env var, or PATH")]
struct Args
{
    #[arg(long = "git-path")]
    git: Option<PathBuf>,

    #[arg(long)]
    print_git: bool,

    #[arg(long)]
    version: bool,
}

fn resolve_git_path(args: &Args) -> PathBuf
{
    args.git.clone()
        .or_else(|| env::var_os("GIT")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("git"))
}

fn run_git_version(git: &PathBuf) -> Result<(), String>
{
    let status = Command::new(git)
        .arg("--version")
        .status()
        .map_err(|error| format!("failed to execute git at '{}': {error}", git.display()))?;

    if !status.success()
    {
        return Err(format!("git exited with non zero status: {status}"));
    }

    Ok(())
}

fn main() -> ExitCode
{
    let args = Args::parse();
    let git = resolve_git_path(&args);

    if args.version
    {
        if let Err(message) = run_git_version(&git)
        {
            eprintln!("{message}");
            return ExitCode::from(1);
        }

        return ExitCode::SUCCESS;
    }

    if args.print_git
    {
        println!("{}", git.display());
        return ExitCode::SUCCESS;
    }

    println!("Resolved git path: {}", git.display());
    ExitCode::SUCCESS
}