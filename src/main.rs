use minigrep::search;
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect(); // turn iter to collection vector

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "----------- Searching for {} in {} ----------- ",
        config.query, config.file_path
    );

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in contents.lines() {
        search(line, config.query);
    }
    Ok(())
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}
impl Config<'_> {
    fn build(args: &'_ [String]) -> Result<Config<'_>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}
