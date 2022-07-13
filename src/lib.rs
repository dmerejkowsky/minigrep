use std::env;
use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];

        let case_sensitive = !env::var("MINIGREP_CASE_INSENSITIVE").is_ok();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines = if config.case_sensitive {
        search(&contents, config.query)
    } else {
        search_insensitive(&contents, config.query)
    };
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .collect()
}
