use std::env;
//need this module in scope so we can use its args function
use std::process;
//to bring process::exit into scope
use minigrep::Config;
//bring the Config type from the library crate (lib.rs) into scope

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
    // To handle an error case in a user friendly way, we will
    // make the main handle the Result returned by Config::new
    // Then create an error message to print whne Result could 
    //not be parsed
    // unwrap_or_else is defined on Result<T, E> by standard library
    //it allows us to define some custom, non-panic error handling
    //If the value is ok, it returns the value and if it is an error,
    // it calls the code in the closure

    if let Err(e) = minigrep::run(config) {
    // using if let to check wether run returns an Err value and
    // call process:exit(1) if it does
        println!("Application error: {}", e);

        process::exit(1);
    }
}