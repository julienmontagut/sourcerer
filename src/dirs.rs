use std::path::PathBuf;

pub fn search_in_parents(path: PathBuf, depth: u32) -> PathBuf {
    let mut current_path = path;
    for _ in 0..depth {
        current_path = current_path.parent().unwrap().to_path_buf();
    }
    current_path
}

pub fn get_root_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_path = std::env::current_dir()?;
    let mut current_path = std::fs::canonicalize(current_path)?;
    let home_dir = home_dir()?;
    while !current_path.join(".srcman").exists()
        && current_path != PathBuf::from("/")
        && current_path != home_dir
    {
        println!("{}", current_path.display());
        if let Some(parent) = current_path.parent() {
            current_path = parent.to_path_buf();
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No .srcman file found in the parent directories",
            )));
        }
    }
    if current_path == PathBuf::from("/") || current_path == home_dir {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No .srcman file found in the parent directories",
        )));
    }
    Ok(current_path)
}

fn home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(std::env::home_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Home directory not found",
    ))?)
}

fn config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    match std::env::var("XDG_CONFIG_HOME") {
        Ok(env_var) => Ok(PathBuf::from(env_var)),
        Err(_) => Ok(home_dir()?.join(".config")),
    }
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(config_dir()?.join("srcman").join("config.toml"))
}
