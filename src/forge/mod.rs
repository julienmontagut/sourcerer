pub mod github;

pub use github::client::Client as Github;

type BoxedError = Box<dyn std::error::Error>;

pub trait Forge {
    fn new(token: &str) -> Self;
    fn get_repos(&self) -> Result<Vec<Repo>, BoxedError>;
    fn get_starred_repos(&self) -> Result<Vec<Repo>, BoxedError>;
    fn get_org_repos(&self) -> Result<Vec<Repo>, BoxedError>;
    fn get_orgs(&self) -> Result<Vec<Org>, BoxedError>;
}

pub struct Repo {
    name: String,
    url: String,
}

pub struct Org {
    name: String,
    url: String,
}

pub struct User {
    name: String,
    url: String,
}
