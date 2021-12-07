mod config;

use std::{env, error::Error, fs::create_dir, process};

use teart::quantizie_image;

use crate::config::Config;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    const SIZE: u32 = 128;
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    create_dir("output").ok();
    quantizie_image(config.path.as_str(), (SIZE, SIZE));

    Ok(())
}
