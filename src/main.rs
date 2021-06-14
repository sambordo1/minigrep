use std::env;
//need this module in scope so we can use its args function
use std::fs;
//to handle files
use std::process;
//to bring process::exit into scope

fn main() {
    let args: Vec<String> = env::args().collect();
    // takes in command line arguments, then calls the collect 
    //method on the iterator to turn it into a vector
    //the collect function can be used to create many types of
    //of args so we need to specify that we want a vector of strings
    // with Vec<String>

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
    // fs::read_to_string takes the filename, opens it, and returns
    // Result<String> of the file's contents
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}