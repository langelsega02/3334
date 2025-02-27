use std::fs::File;
use std::io::prelude::*;

struct Config {
    username: String,
    api_key: String,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let username = lines.next().unwrap().to_string();
        let api_key = lines.next().unwrap().to_string();

        Config { username, api_key }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("username: {}", config.username);
    println!("SID : {}", config.api_key);
}

fn main() {
    reading_from_file();
}
