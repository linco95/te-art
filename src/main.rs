mod config;

use std::{env, error::Error, fs::create_dir, path::Path, process};

use teart::{get_raw_buffer, quantizie_image, save_image};

use crate::config::Config;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    const SIZE: u32 = 128;
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let dimensions = (SIZE, SIZE);

    create_dir("output").ok();
    let quant_res = quantizie_image(config.path.as_str(), dimensions).unwrap();
    let raw_buffer = get_raw_buffer(&quant_res.result_pixels);

    save_image(
        raw_buffer,
        dimensions,
        format!(
            "./output/quantized_output_{}",
            Path::new(config.path.as_str())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .as_str(),
    );
    Ok(())
}
