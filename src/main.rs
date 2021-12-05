use std::env;

use image::{DynamicImage, GenericImageView};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = parse_config(&args);
    println!("Received path: {}", path);

    let img = read_image(path);
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
}

fn read_image(path: &str) -> DynamicImage {
    image::open(path).unwrap()
}

fn parse_config(args: &[String]) -> &str {
    let path = &args[1];

    path
}
