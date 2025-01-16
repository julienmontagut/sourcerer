mod args;
mod conf;
mod dirs;
mod git;
mod github;
mod repo;
use args::{Args, Command};
use clap::Parser;
use conf::Config;
use github::{
    client::Client,
    data::{Repository, User},
};
use std::{env, fs::File, io::Write, process::ExitCode};

fn main() -> ExitCode {
    let Ok(root_dir) = dirs::get_root_dir() else {
        eprintln!("Error: Failed to find the root directory");
        return ExitCode::FAILURE;
    };

    let args = Args::parse();

    let Ok(config) = Config::load(args.config) else {
        eprintln!("Warning: Failed to load config file using default values");
        return ExitCode::FAILURE;
    };

    let token = env::var("GITHUB_TOKEN").ok();

    let command = args.command.unwrap_or(Command::Status);

    match command {
        Command::Init => {
            println!("Init");
        }
        Command::Status => {
            // For each repository in the directory, get the status and print it pretty
            let repos = std::fs::read_dir(root_dir).unwrap();
            for repo in repos {
                match repo {
                    Ok(dir) => {
                        let repo = git::Repository::try_from(dir.path());
                        if let Ok(repo) = repo {
                            let status = repo.status().unwrap();
                            println!("{}: {}", repo.path().display().to_string(), status);
                        } else {
                            eprintln!("Warning: {} is not a git repository", dir.path().display());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            return ExitCode::SUCCESS;
        }
    }

    let Ok(token) = env::var("GITHUB_TOKEN") else {
        eprintln!("Error: GITHUB_TOKEN is required");
        return ExitCode::FAILURE;
    };

    let client = Client::new(token.as_str());
    let octocat = client.get_octocat().unwrap();
    println!("Octocat: {}", octocat);

    if let Ok(user) = client.get::<User>("user") {
        if args.debug {
            println!("{:#?}", user);
        }
    }

    if let Ok(repos) = client.get::<Vec<Repository>>("user/repos") {
        if args.debug {
            println!("{:#?}", repos);
        }
    }

    if let Ok(response) = client.get_user_orgs() {
        if args.debug {
            println!("{}", response);
        }
        if let Some(path) = &args.save {
            let path = format!("{}/orgs.json", path);
            file_write_response(&path, response);
        }
    }

    if let Ok(response) = client.get_user_starred() {
        if args.debug {
            println!("{}", response);
        }
        if let Some(path) = &args.save {
            let path = format!("{}/starred.json", path);
            file_write_response(&path, response);
        }
    }

    if let Ok(response) = client.get_user_watched() {
        if args.debug {
            println!("{}", response);
        }
        if let Some(path) = &args.save {
            let path = format!("{}/watched.json", path);
            file_write_response(&path, response);
        }
    }

    ExitCode::SUCCESS
}

fn file_write_response(path: &str, response: String) {
    match File::create(path) {
        Ok(mut file) => {
            file.write_all(response.as_bytes())
                .expect("Error writing file");
            file.flush().expect("Error flushing file");
        }
        Err(error) => eprintln!("Error creating file: {}", error),
    }
}
