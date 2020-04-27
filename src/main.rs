use minigrep::{self, Config};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)?;
    minigrep::run(&config)
}
