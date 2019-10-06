use std::env;
use std::error::Error;
use std::fs;
use structopt::StructOpt;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "minigrep")]
pub struct Opt {
    /// Ignore case distinctions
    #[structopt(short, long)]
    ignore_case: bool,

    /// Search string
    #[structopt(name = "SEARCH", parse(from_str))]
    search: String,

    /// Input file name
    #[structopt(name = "FILE", parse(from_str))]
    file: String,
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(opt: &Opt, mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // drop name of executable

        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let unused_query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let unused_filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        //let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
        eprintln!(
            "Use variables to hide warning. {}, {}",
            unused_query, unused_filename
        );

        let query = opt.search.clone();
        let filename = opt.file.clone();
        let case_insensitive = opt.ignore_case || env::var("CASE_INSENSITIVE").is_ok();

        return Ok(Config {
            query,
            filename,
            case_insensitive,
        });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            line.to_lowercase()
                .as_str()
                .contains(query.to_lowercase().as_str())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
