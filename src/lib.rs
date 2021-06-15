use std::error::Error;
//to bring Box<dyn Error> into scope
use std::fs;
//to handle files

//using pub keyword to make the library public to use
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // return type is Result<(), Box<dyn Error>>
    // Box<dyn Error> means the function will return 
    //a type that implements the Error trait
    
        let contents = fs::read_to_string(config.filename)?;
    //reads the file to strings, then the ?  will return the error value 
    //from the current function for the caller to handle.
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }    
        Ok(()) //returns ok if successful
    }
    
//Writing a test
// it will take a query and the text to search for the query in,
// and it will return only the lines from the text that contain the query.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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