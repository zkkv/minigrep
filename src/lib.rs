use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;
    
	//.expect("Couldn't read the file in file path. Make sure it exists!");
    println!("With text:\n{contents}");
	Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
	pub fn build(args: &[String]) -> Result<Config, &str> {
		if args.len() < 3 {
            return Err("Usage: cargo run [string] [file-path]");
        }

		let query = args[1].clone();
		let file_path = args[2].clone();
	
		Ok(Config {query, file_path})
	}
}