/* Main file that creates project based on cheriot-template https://github.com/CHERIoT-Platform/cheriot-template */

use clap::Parser;
use std::{path::PathBuf};
use git2::Repository;

#[derive(Parser, Debug)]
#[command(name = "create-project")]
#[command(about = "Generate new template project with specific path. Use --path argument to create new template folder")]
struct Args
{
    #[arg(long = "path")]
    folder: PathBuf,

    #[arg(long)]
    version: bool,
}

fn main () {
    let args = Args::parse();
    let path = args.folder;
    let url_template = "https://github.com/CHERIoT-Platform/cheriot-template";

    assert!(!path.as_path().exists());

    let _repo = match Repository::clone_recurse(url_template, path) {
        Ok(_repo) => println!("Project cloned successfully!"),
        Err(e) => panic!("Failed to clone: {}", e),
    };
}
