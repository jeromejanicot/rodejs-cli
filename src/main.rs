use clap::Parser;
use git2::Repository;
use std::fs::create_dir;

const NODEJS: &str = "https://github.com/jeromejanicot/rodejs-templates.git";


#[derive(Parser)]
struct Cli {
    project_name: String,
    template: String,
    path: std::path::PathBuf

}

fn main() {
    let args = Cli::parse();

    let path = args.path.join(args.project_name);
    match create_dir(&path) {
        Ok(_) => println!("Directory created at {:?}", &path),
        Err(error) => panic!("Failed to create directory: {}", error),
    };

    let repo = match Repository::clone(NODEJS, path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    println!("Repository cloned to {}", repo.path().display());
}
