use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let mut args = env::args();
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}


/// Search and print the matched string according to the input config.
///
/// # Example
///
/// ```
/// use minigrep::run;
/// use minigrep::Config;
///
/// run(Config {
///     filename: "resources/t.txt".to_string(),
///     query: "Fun".to_string(),
///     case_sensitive: false,
/// });
///
/// assert!(true);
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_ignore_case(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

/// Search certain patterns from the given string ignoring the case.
///
/// Just like `minigrep::search`, but ignore the case of `query`.
pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// Search certain patterns from the given string
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

