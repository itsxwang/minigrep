use minigrep::{search_case_insensitive, search_case_smart, search_sensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect(); // turn iter to collection vector

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "SET ENV:\nCASE=0 => For Case INSENITIVE\nCASE=1 => For Case SMART\nCASE=2 (default) => For Case SENSITIVE\n",
    );
    println!(
        "----------- Searching for `{}` in `{}` -----------\n",
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
        match config.case {
            "smart" => {
                if let Some(s) = search_case_smart(line, config.query) {
                    print_matches(s);
                }
            }
            "insensitive" => {
                if let Some(s) = search_case_insensitive(line, config.query) {
                    print_matches(s);
                }
            }
            _ => {
                if let Some(s) = search_sensitive(line, config.query) {
                    print_matches(s);
                }
            }
        }
    }

    Ok(())
}

fn print_matches(s: &str) {
    // for now simply print the lines
    println!("{}", s);
}
struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    case: &'a str,
}
impl Config<'_> {
    fn build<'a>(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];

        let case = match env::var("CASE") {
            Ok(val) => {
                match val.as_str() {
                    "0" => "insensitive",
                    "1" => "smart",
                    _ => "sensitive",
                }
            }
            Err(_) => "sensitive",
        };

        Ok(Config {
            query,
            file_path,
            case,
        })
    }
}
