// Code for a hosting service implemented by gitklab and github that
// allors to list repositories for an organization and clone them locally
trait GitHostingService {
    fn get_repos(&self, org: &str) -> Result<Vec<Repository>, Error>;
    fn clone_repos(&self, org: &str, repos: Vec<Repository>) -> Result<(), Error>;
}

trait GitRepo {
    fn clone(&self) -> Result<(), Error>;
    fn pull(&self) -> Result<(), Error>;
}

struct Repository {
    name: String,
    url: String,
    path: String,
}

trait Source {
    fn clone(&self) -> Result<(), Error>;
    fn pull(&self) -> Result<(), Error>;
}

trait Owner {
    fn get_repos(&self) -> Result<Vec<Repository>, Error>;
}
