mod config;

use std::{
    env,
    error::Error,
    fs::{create_dir, write},
    path::Path,
    process,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use teart::{
    image_parsing::{get_palette, get_raw_buffer, parse_image, save_image, InputData},
    reservation_converter::{convert_image_to_reservation, ServerParams, TEObject, DATE_FORMAT},
};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    login_name: String,
    size: u32,
    start_datetime: String,
    reservation_mode: String,
    org: String,
    canvas_object: (String, String),
    color_objects: Vec<(String, String)>,
    auth_server: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            size: 128,
            login_name: "".to_string(),
            auth_server: "timeedit".to_string(),
            org: "admin".to_string(),
            reservation_mode: "coloring".to_string(),
            start_datetime: "20220103T000000".to_string(),
            canvas_object: ("canvas".to_string(), "linn".to_string()),
            color_objects: (0..get_palette().len())
                .into_iter()
                .map(|i| ("color_object".into(), format!("color_object_{}", i)))
                .collect(),
        }
    }
}

impl From<AppConfig> for ServerParams {
    fn from(cfg: AppConfig) -> Self {
        if cfg.login_name.is_empty() {
            panic!("Login name was not found, please add it to teart_cfg")
        }
        if cfg.color_objects.len() != get_palette().len() - 1 {
            panic!("Must be exactly {} color objects", get_palette().len() - 1)
        }
        let naive_start_datetime =
            NaiveDateTime::parse_from_str(&cfg.start_datetime, DATE_FORMAT).unwrap();
        ServerParams {
            login_name: cfg.login_name,
            auth_server: cfg.auth_server,
            org: cfg.org,
            reservation_mode: cfg.reservation_mode,
            canvas_object: TEObject::new(
                cfg.canvas_object.0.as_str(),
                cfg.canvas_object.1.as_str(),
            ),
            color_objects: cfg
                .color_objects
                .iter()
                .map(|(type_id, object_id)| TEObject::new(type_id.as_str(), object_id.as_str()))
                .collect(),
            start_datetime: DateTime::<Utc>::from_utc(naive_start_datetime, Utc),
        }
    }
}

const HELP_STR: &'static str = r#"
╔╦╗╔═╗  ╔═╗╦═╗╔╦╗
 ║ ║╣   ╠═╣╠╦╝ ║ 
 ╩ ╚═╝  ╩ ╩╩╚═ ╩ 
Developed by: Andreas Kjellqvist.
Special thanks to Göran Hällgren, Fredrik Björeman for help with XML importer
****************************************************
Welcome to my TE Art project!
The first time you run this program it will crash as you need to fill out the config file!
To run the tool just update the config file (teart_cfg) and run it with path to the image you want to create. 
After it is done converting the image it will provide a preview in output folder. 
If the preview looks good you can use the XML Importer to import the reservations into TimeEdit. 
Just select the xml payload file in output folder as import file.
****************************************************
The config file contains of the following parameters:
# Username of the user importing the reservations
login_name: String, 
# The size of the final image, works best with increments of 2 (2, 4, 8, 16, 32, 64, 128)
# (recommended max 128 as that's the most that fits in 24 hours)
size: u32,              
# The starting datetime for the reservations. Recommended to be 00:00. This will be converted to local timezone of the importing user
start_datetime: String,
# Reservation mode for the import
reservation_mode: String,
# Organization to do the reservations for
org: String,
# This object is added to all reservations. This is what is selected to view the painting. You could have multiple drawings in the same
# TE database by having each drawing on a canvas object, and swich which canvas object is selected
canvas_object: (String, String),
# List of the color objects. These are expected to be colored in TimeEdit, in the same order as the list of solid colors, and with the
# last object being uncolored (for grey color)
color_objects: Vec<(String, String)>,
# For xml_importer this should be timeedit
auth_server: String,
****************************************************"#;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    println!("{}", HELP_STR);
    let cfg: AppConfig = confy::load_path("./teart_cfg")?;

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let dimensions = (cfg.size, cfg.size);
    let server_params: ServerParams = cfg.into();

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

    let xml_payload = convert_image_to_reservation(
        quant_res.quantized_image.into_raw_vec(),
        dimensions,
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
