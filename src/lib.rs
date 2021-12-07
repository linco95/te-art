use std::{error::Error, path::Path};

use rscolorq::{color::Rgb, spatial_color_quant, Matrix2d, Params};

use image::{
    imageops::{resize, FilterType},
    save_buffer, ColorType, DynamicImage, ImageBuffer, Pixel, Rgba,
};
pub struct QuantizationResult {
    pub result_pixels: Vec<Rgb>,
    pub quantized_image: Matrix2d<u8>,
}

pub fn get_palette() -> Vec<Rgb> {
    Vec::from([
        [1, 1, 1],
        [172, 218, 148],
        [255, 254, 231],
        [147, 185, 238],
        [216, 205, 225],
        [199, 235, 224],
        [246, 180, 81],
        [249, 152, 131],
        [170, 216, 232],
        [255, 234, 150],
        [250, 214, 222],
        [216, 231, 53],
        [171, 167, 125],
        [200, 112, 112],
        [201, 255, 146],
        [230, 181, 219],
        [219, 192, 163],
    ])
    .iter()
    .map(|&c| into_f64_rgb(&c))
    .collect()
}

pub fn quantizie_image(
    path: &str,
    dimensions: (u32, u32),
) -> std::result::Result<QuantizationResult, Box<dyn Error>> {
    let path = Path::new(path);
    let img = read_image(path.to_str().unwrap());
    let scaled_img = scale_image(img, dimensions);
    let pixels = get_pixels(&scaled_img);

    quantize_image(
        dimensions.0 as usize,
        dimensions.1 as usize,
        pixels,
        get_palette(),
    )
}

pub fn get_raw_buffer(pixels: &[Rgb]) -> Vec<u8> {
    pixels
        .iter()
        .flat_map(move |pixel| into_u8_rgb(*pixel))
        .collect()
}

pub fn save_image(buffer: Vec<u8>, dimensions: (u32, u32), path: &str) {
    save_buffer(
        path,
        buffer.as_slice(),
        dimensions.0,
        dimensions.1,
        ColorType::Rgb8,
    )
    .expect("Failed to save image buffer");
}

fn scale_image(img: DynamicImage, dimensions: (u32, u32)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    resize(&img, dimensions.0, dimensions.1, FilterType::Nearest)
}

fn get_pixels(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<&[u8]> {
    img.pixels().map(|pixel| pixel.channels()).collect()
}

fn into_f64_rgb(pixel: &[u8]) -> Rgb {
    Rgb {
        red: pixel[0] as f64 / 255.,
        green: pixel[1] as f64 / 255.,
        blue: pixel[2] as f64 / 255.,
    }
}

fn into_u8_rgb(pixel: Rgb) -> [u8; 3] {
    [
        (pixel.red * 255.) as u8,
        (pixel.blue * 255.) as u8,
        (pixel.green * 255.) as u8,
    ]
}

fn read_image(path: &str) -> DynamicImage {
    // TODO: Add error handling?
    image::open(path).unwrap()
}

fn quantize_image(
    width: usize,
    height: usize,
    img: Vec<&[u8]>,
    input_palette: Vec<Rgb>,
) -> Result<QuantizationResult, Box<dyn Error>> {
    println!("Quantizizing image...");

    // Build the quantization parameters, verify if accepting user input
    let palette_size = input_palette.len() as u8;
    let mut conditions = Params::new();
    conditions.palette_size(palette_size);
    conditions.dithering_level_auto(width as u32, height as u32, palette_size as usize);
    conditions.palette(input_palette)?;
    conditions.verify_parameters()?;

    // Convert the input image buffer from Rgb<u8> to Rgb<f64>
    let image = Matrix2d::from_vec(
        img.iter().map(|&c| into_f64_rgb(c)).collect(),
        width,
        height,
    );

    let mut palette = Vec::with_capacity(palette_size as usize);

    // Create the output buffer and quantized palette index buffer
    let mut quantized_image = Matrix2d::new(width, height);

    spatial_color_quant(&image, &mut quantized_image, &mut palette, &conditions)?;

    let result_pixels = quantized_image
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
    Ok(QuantizationResult {
        result_pixels,
        quantized_image,
    })
}
