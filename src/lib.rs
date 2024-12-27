use std::error::Error;
use std::fs;

const HELP: &str = "\
Usage: minigrep [PATTERN] [FILE_PATH]
Search for a pattern in the given file

Flags:
    -h, --help          display this help message
    -i, --ignore_case   ignore case distinctions in patterns
";

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub help: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
            return Ok(Config {
                query: "".to_string(),
                file_path: "".to_string(),
                ignore_case: false,
                help: true,
            });
        }
        if args.len() < 3 {
            return Err("minigrep [PATTERN] [FILE_PATH]");
        }

        let mut query = String::new();
        let mut file_path = String::new();

        let mut ignore_case = false;

        let mut args_counter = 2;
        for arg in args {
            // con un match è più facile espendare le flag del CLI
            match &arg[..] {
                "-h" | "--help" => {
                    return Ok(Config {
                        query: "".to_string(),
                        file_path: "".to_string(),
                        ignore_case: false,
                        help: true,
                    });
                }
                "-i" | "--ignore_case" => ignore_case = true,
                // controllo dei valori non flag
                _ => {
                    match args_counter {
                        // il primo valore del vettore args non serve
                        // ai nostri scopi in questo momento
                        2 => args_counter -= 1,
                        1 => {
                            query = arg.clone();
                            args_counter -= 1;
                        }
                        0 => {
                            file_path = arg.clone();
                            args_counter -= 1;
                        }
                        // caso in cui sono stati immessi troppi parametri
                        _ => return Err("Too many parameters"),
                    }
                }
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
            help: false,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.help {
        println!("{HELP}");
        return Ok(());
    }

    let content = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "fast";
        let content = "\
Rust:
safe, fast and productive
pick three.";

        assert_eq!(vec!["safe, fast and productive"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast and productive
pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }

    //#[test]
    //fn arguments() {
    //    let args = &["to", "poem.txt", "-i"];
    //    let args = &[String];
    //
    //    assert_eq!(
    //        Ok(Config {
    //            query: "to".to_string(),
    //            file_path: "poem.txt".to_string(),
    //            ignore_case: true
    //        }),
    //        Config::new(args)
    //    )
    //}
}
