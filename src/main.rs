mod config;

use std::{
    env,
    error::Error,
    fs::{create_dir, write},
    path::Path,
    process,
};

use chrono::{TimeZone, Utc};
use teart::{
    image_parsing::{get_palette, get_raw_buffer, parse_image, save_image, InputData},
    reservation_converter::{
        convert_image_to_reservation, get_default_color_objects, ServerParams, TEObject,
    },
};

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
    let quant_res = parse_image(InputData {
        path: config.path.clone(),
        dimensions,
    })
    .unwrap();
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

    let login_name = "TEAdmin_ak".to_string();
    let auth_server = "timeedit".to_string();
    let org = "admin".to_string();
    let reservation_mode = "StudentGroupRoomBooking".to_string();
    let start_datetime = Utc.ymd(2022, 1, 1).and_hms_milli(0, 0, 0, 0);
    let canvas_object = TEObject::new("canvas", "_te_411193");
    let color_objects = get_default_color_objects(get_palette().len() as u8);

    let server_params = ServerParams {
        login_name,
        auth_server,
        org,
        reservation_mode,
        canvas_object,
        color_objects,
    };

    let xml_payload = convert_image_to_reservation(
        quant_res.quantized_image.into_raw_vec(),
        dimensions,
        start_datetime,
        server_params,
    );

    write(
        format!(
            "output/xml_payload_{}.xml",
            Path::new(config.path.as_str())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        ),
        xml_payload?,
    )?;
    Ok(())
}
