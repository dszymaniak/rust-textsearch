use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?; //zamiast .expect("failed to read the file")
    //println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };


    for line in results {
        println!("{}",line);
    }
    
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> { //struct Config zamiast (&str,&str)
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone(); // nie chce miec ownership tego stringa wiec jest to zamist &args[1]
        let filename: String = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {query,filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_sensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
        Rust:
safe,fast,productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe,fast,productive."], search(query,contents));
    }

    #[test]
    fn case_sensitive() {
        let query: &str = "rUsT";
        let contents: &str = "\
        Rust:
safe,fast,productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."],
        search_case_sensitive(query,contents));
    }
}