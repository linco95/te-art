use std::ops::Add;

use chrono::{DateTime, Duration, Utc};

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(i: usize, dimensions: (u32, u32)) -> Point {
        Point {
            x: i as u32 % dimensions.1,
            y: i as u32 / dimensions.0,
        }
    }
}

trait XMLFormat {
    fn to_xml(&self) -> String;
}
#[derive(Debug, Clone)]
pub struct TEObject {
    type_id: String,
    object_id: String,
}

impl TEObject {
    pub fn new(type_id: &str, id: &str) -> TEObject {
        TEObject {
            type_id: type_id.to_string(),
            object_id: id.to_string(),
        }
    }
}

impl XMLFormat for TEObject {
    fn to_xml(&self) -> String {
        format!(
            "<object><type>{}</type><value>{}</value></object>",
            self.type_id, self.object_id
        )
    }
}

struct TEField {
    field_id: String,
    value: String,
}

impl XMLFormat for TEField {
    fn to_xml(&self) -> String {
        format!(
            "<field><extid>{}</extid><value>{}</value></field>",
            self.field_id, self.value
        )
    }
}

struct Reservation<'a> {
    start_time: DateTime<Utc>, // start timestamp
    objects: Vec<&'a TEObject>,
    fields: Vec<TEField>,
    start_time_offset: Duration, // Offset from start timestamp
    duration: Duration,
    org: String,
}
pub const DATE_FORMAT: &str = "%Y%m%dT%H%M00";

pub struct ServerParams {
    pub login_name: String,
    pub auth_server: String,
    pub org: String,
    pub reservation_mode: String,
    pub canvas_object: TEObject,
    pub color_objects: Vec<TEObject>,
    pub start_datetime: DateTime<Utc>,
}

impl XMLFormat for Reservation<'_> {
    fn to_xml(&self) -> String {
        format!(
            "<reservation><begin>{}</begin><end>{}</end><objects>{}</objects><fields>{}</fields><organizations><organization>{}</organization></organizations></reservation>",
            self.start_time.add(self.start_time_offset).format(DATE_FORMAT),
            self.start_time
                .add(self.start_time_offset)
                .add(self.duration).format(DATE_FORMAT),
            self.objects
                .iter()
                .map(|object| { object.to_xml() })
                .collect::<String>(),
            self.fields
                .iter()
                .map(|field| { field.to_xml() })
                .collect::<String>(),
                self.org
        )
    }
}

pub fn convert_image_to_reservation(
    image: Vec<u8>,
    dimensions: (u32, u32),
    server_params: ServerParams,
) -> Result<String, String> {
    let reservations: Vec<Reservation> = image
        .iter()
        .enumerate()
        .map(|(i, color)| {
            convert_pixel_to_reservation(
                *color,
                Point::new(i, dimensions),
                server_params.start_datetime,
                10,
                server_params.org.as_str(),
                &server_params.color_objects,
                &server_params.canvas_object,
            )
        })
        .flatten()
        .collect();

    Ok(create_xml_payload(reservations))
}

pub fn get_default_color_objects(palette_size: u8) -> Vec<TEObject> {
    (0..palette_size)
        .into_iter()
        .map(|idx| TEObject::new("color_object", format!("object_color_{}", idx).as_str()))
        .collect()
}

fn create_xml_payload(reservations: Vec<Reservation>) -> String {
    format!(
        "<tns:timeedit>{}</tns:timeedit>",
        reservations
            .iter()
            .map(|reservation| reservation.to_xml())
            .collect::<Vec<String>>()
            .join(""),
    )
}

fn convert_pixel_to_reservation<'a>(
    color: u8,
    coord: Point,
    start_time: DateTime<Utc>,
    timestep: u32,
    org: &str,
    color_objects: &'a [TEObject],
    canvas_object: &'a TEObject,
) -> Result<Reservation<'a>, String> {
    if color >= color_objects.len() as u8 + 1 {
        return Err(format!("Color was out of range: {}", color));
    } else if color == color_objects.len() as u8 {
        return Ok(Reservation {
            start_time,
            objects: vec![color_objects.get(11).unwrap(), canvas_object],
            fields: vec![],
            start_time_offset: Duration::minutes(
                (Duration::days(coord.x.into()).num_minutes() as u32 + coord.y * timestep) as i64,
            ),
            duration: Duration::minutes(timestep as i64),
            org: org.to_string(),
        });
    } else {
        return Ok(Reservation {
            start_time,
            objects: vec![color_objects.get(color as usize).unwrap(), canvas_object],
            fields: vec![],
            start_time_offset: Duration::minutes(
                (Duration::days(coord.x.into()).num_minutes() as u32 + coord.y * timestep) as i64,
            ),
            duration: Duration::minutes(timestep as i64),
            org: org.to_string(),
        });
    }
}
