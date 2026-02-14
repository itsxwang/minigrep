use minigrep::{search_case_insensitive, search_case_smart, search_sensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect(); // turn iter to collection vector

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "SET ENV OR SET ARGS(more priority):\nCASE=0 => For Case INSENITIVE\nCASE=1 => For Case SMART\nCASE=2 (default) => For Case SENSITIVE\n",
    );
    println!(
        "----------- Searching for `{}` in `{}` -----------\n",
        config.query, config.file_path
    );

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in contents.lines() {
        match config.case {
            "1" => {
                if let Some(s) = search_case_smart(line, config.query) {
                    print_matches(s);
                }
            }
            "0" => {
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
        let case: &str;
        let query: &str;
        let file_path: &str;

        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 3 {
            query = &args[1];
            file_path = &args[2];
            case = &args[3];
        } else {
            query = &args[1];
            file_path = &args[2];

            case = match env::var("CASE") {
                Ok(val) => match val.as_str() {
                    "0" => "0",
                    "1" => "1",
                    _ => "2",
                },
                Err(_) => "2",
            };
        }

        Ok(Config {
            query,
            file_path,
            case,
        })
    }
}
