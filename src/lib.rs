mod diagnosis;

use std::env;

pub struct Config {
    pub command: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Usage: doc <command>")
        };
        Ok(Config{command})
    }
}