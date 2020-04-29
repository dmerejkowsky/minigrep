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
    let lw_query = config.query.to_lowercase();
    let match_fn: Box<dyn Fn(&str) -> bool> = if config.case_sensitive {
        Box::new(|line| line.contains(&config.query))
    } else {
        Box::new(|line| line.to_lowercase().contains(&lw_query))
    };
    for line in search(&contents, match_fn) {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a, T>(contents: &'a str, matcher: T) -> Vec<&'a str>
where
    T: Fn(&str) -> bool,
{
    let mut res = vec![];
    for line in contents.lines() {
        if matcher(line) {
            res.push(line)
        }
    }
    res
}
