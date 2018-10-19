extern crate git2;
extern crate url;

use git2::Repository;
use std::env;
use std::string::String;
use url::Url;

fn get_remote<'a>(repo: &'a git2::Repository) -> String {
    let res = repo.find_remote("origin");
    match res {
        Ok(r) => return r.url().unwrap().to_string(),
        Err(_e) => panic!("Can't find remote"),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let repo = match Repository::open(&args[1]) {
        Ok(repo) => repo,
        Err(_e) => panic!("Not a git repo"),
    };
    let res = get_remote(&repo);
    let remote_url_res = Url::parse(res.as_str());
    let remote_url = match remote_url_res {
        Ok(r) => r,
        Err(_e) => panic!("Invalid remote url"),
    };
    if remote_url.host_str() == Some("github.com") {
        println!("it's github");
    } else if remote_url.host_str() == Some("bitbucket.org") {
        println!("it's bitbucket");
    };
    println!("{:?}", remote_url.path());
}
