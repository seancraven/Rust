use std::error::Error;
use std::fs;

// Defines acceptable types of argument user can input.
pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    // Result enum is a I am prepared for this error.
    // 'static indicates a lifetime for the whole program.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Requires two arguments, query and ./file/path");
        }
        // pass a reference to the slice object,
        // Clone the values found at the slice object's index
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n{contents}");

    return Ok(());
}
