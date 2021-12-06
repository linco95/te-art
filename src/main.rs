mod config;

use std::{env, error::Error, fs::create_dir, process};

use image::GenericImageView;
use teart::{parse_image, read_image};

use crate::config::Config;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    const SIZE: u32 = 128;
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let img = read_image(config.path.as_str());

    // The dimensions method returns the images width and height.
    println!("dimensions: {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("Colortype: {:?}", img.color());
    println!("Ensuring output folder exists");
    create_dir("output").ok();

    parse_image(img, SIZE, SIZE);

    Ok(())
}
