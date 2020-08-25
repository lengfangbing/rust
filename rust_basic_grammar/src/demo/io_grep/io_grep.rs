use std::{env, process};
use std::fs;
use std::error::Error;
use std::time::Instant;


#[derive(Debug)]
struct Config {
    search: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            Err("not enough arguments")
        } else {
            let search = args.next().unwrap_or_else(|| {
                eprintln!("search is invalid");
                process::exit(1);
            });
            let filename = args.next().unwrap_or_else(|| {
                eprintln!("filename is invalid");
                process::exit(1);
            });
            let case_sensitive = env::var("CASE_SENSITIVE")
                .is_err();
            Ok(Config {
                search,
                filename,
                case_sensitive,
            })
        }
    }
}

fn get_own_args() -> env::Args {
    let mut args = env::args().into_iter();
    args.next();
    args
}

fn get_txt(filename: String) -> (bool, String) {
    let content = fs::read_to_string(&filename);
    if content.is_ok() {
        (true, content.unwrap())
    } else {
        (false, format!("The path to read_to_string: {} is invalid\nreason is : {}", filename, content.unwrap_err()))
    }
}

fn run(config: Config) -> Result<(), String> {
    let (valid, text) = get_txt(config.filename);
    if valid {
        println!("{}", text);
        Ok(())
    } else {
        Err(text)
    }
}

pub fn start_io_grep() {
    let now = Instant::now();
    let mut own_args = get_own_args();
    let config = Config::new(own_args).unwrap_or_else(|err| {
        eprintln!("Problem invalid arguments {}", err);
        process::exit(1);
    });
    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
    println!("{}", now.elapsed().as_micros());
}
