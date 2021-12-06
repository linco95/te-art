use std::error::Error;

use rscolorq::{color::Rgb, spatial_color_quant, Matrix2d, Params};

use image::{
    imageops::{resize, FilterType},
    save_buffer, ColorType, DynamicImage, Pixel,
};

pub fn parse_image(img: DynamicImage, width: u32, height: u32) {
    let scaled_img = resize(&img, width, height, FilterType::Nearest);
    scaled_img.save("output/scaled_image.png").ok();
    let pixels: Vec<&[u8]> = scaled_img.pixels().map(|pixel| pixel.channels()).collect();

    let result_img = quantify_image(width as usize, height as usize, pixels).unwrap();
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

pub fn read_image(path: &str) -> DynamicImage {
    image::open(path).unwrap()
}

fn quantify_image(
    width: usize,
    height: usize,
    img: Vec<&[u8]>,
) -> Result<Vec<Rgb>, Box<dyn Error>> {
    println!("Quantifying image...");

    let te_palette: Vec<Rgb> = Vec::from([
        (1, 1, 1),
        (172, 218, 148),
        (255, 254, 231),
        (147, 185, 238),
        (216, 205, 225),
        (199, 235, 224),
        (246, 180, 81),
        (249, 152, 131),
        (170, 216, 232),
        (255, 234, 150),
        (250, 214, 222),
        (216, 231, 53),
        (171, 167, 125),
        (200, 112, 112),
        (201, 255, 146),
        (230, 181, 219),
        (219, 192, 163),
    ])
    .iter()
    .map(|&c| Rgb {
        red: c.0 as f64 / 255.0,
        green: c.1 as f64 / 255.0,
        blue: c.2 as f64 / 255.0,
    })
    .collect();

    let palette_size = te_palette.len() as u8;
    // Create the output buffer and quantized palette index buffer
    let mut quantized_image = Matrix2d::new(width, height);

    // Build the quantization parameters, verify if accepting user input
    let mut conditions = Params::new();
    conditions.palette_size(palette_size);
    conditions.dithering_level_auto(width as u32, height as u32, palette_size as usize);
    conditions.palette(te_palette)?;
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
