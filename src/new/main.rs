use anyhow::{Context, Result};
use clap::Parser;
use console::style;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use octocrab::Octocrab;
use url::Url;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

mod config;
use config::Config;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// GitHub personal access token
    #[arg(short, long, env("GITHUB_TOKEN"))]
    token: String,

    /// Path to config file
    #[arg(short, long, default_value = "~/.config/github-sync/config.toml")]
    config: String,

    /// Directory to sync repositories to (overrides config file)
    #[arg(short, long)]
    directory: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load or create config
    let config_path = expand_tilde(&cli.config)?;
    let mut config = if config_path.exists() {
        Config::load(&config_path)?
    } else {
        Config::create_default_file(&config_path)?;
        Config::load(&config_path)?
    };

    // Override sync directory if provided in CLI
    if let Some(dir) = cli.directory {
        config.sync_directory = dir;
    }

    let sync_dir = expand_tilde(&config.sync_directory)?;
    let token = cli.token;

    // Create sync directory if it doesn't exist
    std::fs::create_dir_all(&sync_dir)?;

    // Initialize GitHub client
    let octocrab = Octocrab::builder()
        .personal_token(token.clone())
        .build()
        .context("Failed to create GitHub client")?;

    // Get user info if username is not set
    if config.username.is_empty() {
        let user = octocrab.current().user().await?;
        config.username = user.login;
        config.save(&config_path)?;
    }

    // Get all repositories (user's and from organizations)
    let mut repos = get_all_repositories(&octocrab).await?;
    
    // Get repositories from organizations
    for org in &config.organizations {
        let org_repos = get_organization_repositories(&octocrab, org).await?;
        repos.extend(org_repos);
    }

    // Remove duplicates
    let mut unique_repos: HashSet<_> = HashSet::new();
    repos.retain(|repo| unique_repos.insert(repo.clone_url.clone()));

    println!("{} Found {} repositories", style("✓").green(), repos.len());

    let pb = ProgressBar::new(repos.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} 🧙 {msg}")
            .unwrap()
            .progress_chars("✨⭐-"),
    );

    for repo in repos {
        pb.set_message(format!("Conjuring {}", repo.name));
        
        let repo_path = sync_dir.join(&repo.name);
        let repo_url = repo.clone_url.unwrap();
        if repo_path.exists() {
            update_repository(&repo_path, &repo_url, &token, &config.branch_patterns)?;
        } else {
            clone_repository(&repo_path, &repo_url, &token)?;
        }
        
        pb.inc(1);
    }

    pb.finish_with_message("✨ All repositories have been successfully conjured!");
    Ok(())
}

async fn get_all_repositories(octocrab: &Octocrab) -> Result<Vec<octocrab::models::Repository>> {
    let mut page = 1u8;
    let mut all_repos = Vec::new();
    
    loop {
        let repos = octocrab
            .current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .page(page)
            .send()
            .await?;

        if repos.items.is_empty() {
            break;
        }

        all_repos.extend(repos.items);
        page += 1;
    }

    Ok(all_repos)
}

async fn get_organization_repositories(octocrab: &Octocrab, org: &str) -> Result<Vec<octocrab::models::Repository>> {
    let mut page = 1u32;
    let mut all_repos = Vec::new();
    
    loop {
        let repos = octocrab
            .orgs(org)
            .list_repos()
            .per_page(100)
            .page(page)
            .send()
            .await?;

        if repos.items.is_empty() {
            break;
        }

        all_repos.extend(repos.items);
        page += 1;
    }

    Ok(all_repos)
}

fn clone_repository(path: &Path, url: &Url, token: &str) -> Result<Repository> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username, _allowed_types| {
            Cred::userpass_plaintext("git", token)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    builder
        .clone(url.as_str(), path)
        .context("Failed to clone repository")
}

fn update_repository(path: &Path, url: &Url, token: &str, branch_patterns: &[String]) -> Result<()> {
    let _ = url;
    let repo = Repository::open(path)?;
    let mut remote = repo.find_remote("origin")?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username, _allowed_types| {
        Cred::userpass_plaintext("git", token)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    remote.fetch(branch_patterns, Some(&mut fetch_options), None)?;
    Ok(())
}

fn expand_tilde(path: &str) -> Result<PathBuf> {
    if path.starts_with("~/") {
        let home = dirs::home_dir().context("Could not find home directory")?;
        Ok(home.join(&path[2..]))
    } else {
        Ok(PathBuf::from(path))
    }
}
