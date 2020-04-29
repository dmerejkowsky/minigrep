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
    let query = &config.query;
    if config.case_sensitive {
        SimpleMatcher::new(query).search_and_print(&contents);
    } else {
        CaseInsensitiveMacher::new(query).search_and_print(&contents);
    }
    Ok(())
}

trait LineMatcher {
    fn line_matches(&self, line: &str) -> bool;
}

trait SearchPrinter {
    fn search_and_print(&self, contents: &str);
}

impl<T: LineMatcher> SearchPrinter for T {
    fn search_and_print(&self, contents: &str) {
        for line in contents.lines() {
            if self.line_matches(line) {
                println!("{}", line);
            }
        }
    }
}

struct SimpleMatcher {
    query: String,
}

impl SimpleMatcher {
    fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
        }
    }
}

impl LineMatcher for SimpleMatcher {
    fn line_matches(&self, line: &str) -> bool {
        line.contains(&self.query)
    }
}

struct CaseInsensitiveMacher {
    query: String,
}

impl CaseInsensitiveMacher {
    fn new(query: &str) -> Self {
        Self {
            query: query.to_lowercase(),
        }
    }
}

impl LineMatcher for CaseInsensitiveMacher {
    fn line_matches(&self, line: &str) -> bool {
        line.to_lowercase().contains(&self.query)
    }
}
