use std::error::Error;

use reqwest::blocking::Client as HttpClient;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use serde_json::from_str;

// The github module where the GitHub Client resides
pub struct Client {
    base_url: String,
    client: HttpClient,
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {
            base_url: "https://api.github.com".to_string(),
            client: HttpClient::new(),
            token: token.to_string(),
        }
    }

    pub fn get<T>(&self, path: &str) -> Result<T, Box<dyn Error>>
    where
        for<'a> T: Deserialize<'a> + std::fmt::Debug,
    {
        let url = format!("{}/{}", self.base_url, path);
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .header(USER_AGENT, env!("CARGO_PKG_NAME"))
            .header(ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        let response = request.send()?;
        let json = response.text()?;
        let data = match from_str(json.as_str()) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(Box::new(e));
            }
        };
        println!("{:#?}", data);
        Ok(data)
    }

    fn get_json(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, path);
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .header(USER_AGENT, env!("CARGO_PKG_NAME"))
            .header(ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        let response = request.send()?;
        Ok(response.text()?)
    }

    pub fn get_user_repos(&self) -> Result<String, Box<dyn Error>> {
        self.get_json("user/repos")
    }

    pub fn get_user_orgs(&self) -> Result<String, Box<dyn Error>> {
        self.get_json("user/orgs")
    }

    pub fn get_user_starred(&self) -> Result<String, Box<dyn Error>> {
        self.get_json("user/starred")
    }

    pub fn get_user_watched(&self) -> Result<String, Box<dyn Error>> {
        self.get_json("user/subscriptions")
    }

    pub fn get_octocat(&self) -> Result<String, Box<dyn Error>> {
        self.get_json("octocat")
    }
}
