use rscolorq::{color::Rgb, spatial_color_quant, Matrix2d, Params};
use std::{env, error::Error, fs::create_dir};

use image::{
    imageops::{resize, FilterType},
    save_buffer, ColorType, DynamicImage, GenericImageView, Pixel,
};

fn main() -> std::result::Result<(), Box<dyn Error>> {
    const SIZE: u32 = 64;
    let args: Vec<String> = env::args().collect();

    let path = parse_config(&args);
    println!("Received path: {}", path);

    let img = read_image(path);
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    create_dir("output").ok();

    parse_image(img, SIZE, SIZE);

    Ok(())
}

fn parse_image(img: DynamicImage, width: u32, height: u32) {
    let scaled_img = resize(&img, width, height, FilterType::Nearest);
    let pixels: Vec<&[u8]> = scaled_img.pixels().map(|pixel| pixel.channels()).collect();

    let result_img = quantify_image(width as usize, height as usize, 8, pixels).unwrap();
    println!("result img: {}", (result_img.len() as f32).sqrt());

    let raw_image: Vec<u8> = result_img
        .iter()
        .flat_map(move |color| {
            [
                (color.red * 255.) as u8,
                (color.blue * 255.) as u8,
                (color.green * 255.) as u8,
            ]
        })
        .collect();

    save_buffer(
        "./output/quantized_output.png",
        raw_image.as_slice(),
        width as u32,
        height as u32,
        ColorType::Rgb8,
    )
    .expect("Failed to save image buffer");
}

fn read_image(path: &str) -> DynamicImage {
    image::open(path).unwrap()
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn quantify_image(
    width: usize,
    height: usize,
    palette_size: u8,
    img: Vec<&[u8]>,
) -> Result<Vec<Rgb>, Box<dyn Error>> {
    println!("Quantifying image...");
    // Create the output buffer and quantized palette index buffer
    let mut quantized_image = Matrix2d::new(width, height);

    // Build the quantization parameters, verify if accepting user input
    let mut conditions = Params::new();
    conditions.palette_size(palette_size);
    conditions.verify_parameters()?;

    // Convert the input image buffer from Rgb<u8> to Rgb<f64>
    let image = Matrix2d::from_vec(
        img.iter()
            .map(|&c| Rgb {
                red: c[0] as f64 / 255.0,
                green: c[1] as f64 / 255.0,
                blue: c[2] as f64 / 255.0,
            })
            .collect(),
        width,
        height,
    );

    let mut palette = Vec::with_capacity(palette_size as usize);

    spatial_color_quant(&image, &mut quantized_image, &mut palette, &conditions)?;

    let result_pizels = quantized_image
        .iter()
        .map(move |&color_index| {
            *palette
                .get(color_index as usize)
                .ok_or(format!(
                    "Could not retrieve color {} from palette",
                    color_index
                ))
                .unwrap()
        })
        .collect();
    Ok(result_pizels)
}
