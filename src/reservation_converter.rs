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
struct TEObject {
    type_id: String,
    object_id: String,
}

impl TEObject {
    fn new(id: &str) -> TEObject {
        TEObject {
            type_id: String::from("color_object"),
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

struct Reservation {
    start_time: DateTime<Utc>, // start timestamp
    objects: Vec<TEObject>,
    fields: Vec<TEField>,
    start_time_offset: Duration, // Offset from start timestamp
    duration: Duration,
    org: String,
}

pub struct ServerParams {
    pub login_name: String,
    pub auth_server: String,
    pub org: String,
    pub reservation_mode: String,
}

impl XMLFormat for Reservation {
    fn to_xml(&self) -> String {
        const DATE_FORMAT: &str = "%Y%m%dT%H%M00";
        format!(
            "<reservation><begin>{}</begin><end>{}</end><objects><object><type>canvas</type><value>_te_411193</value></object>{}</objects><fields>{}</fields><organizations><organization>{}</organization></organizations></reservation>",
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
    start_datetime: DateTime<Utc>,
    server_params: ServerParams,
) -> String {
    let reservations: Vec<Reservation> = image
        .iter()
        .enumerate()
        .map(|(i, color)| {
            convert_pixel_to_reservation(
                *color,
                Point::new(i, dimensions),
                Some(10),
                server_params.org.as_str(),
                start_datetime,
            )
        })
        .flatten()
        .collect();

    create_xml_payload(reservations)
}

fn create_xml_payload(reservations: Vec<Reservation>) -> String {
    format!(
        r#"<tns:timeedit>
    {}   
</tns:timeedit>"#,
        reservations
            .iter()
            .map(|reservation| reservation.to_xml())
            .collect::<Vec<String>>()
            .join(""),
    )
}

fn convert_pixel_to_reservation(
    color: u8,
    coord: Point,
    timestep: Option<u32>,
    org: &str,
    start_datetime: DateTime<Utc>,
) -> Option<Reservation> {
    let timestep = timestep.unwrap_or(10);
    let color_objects: Vec<String> = (0..19).map(|n| format!("object_color_{}", n - 1)).collect();
    if color < 1 {
        return Some(Reservation {
            start_time: start_datetime,
            objects: vec![TEObject::new("object_color_11")],
            fields: vec![],
            start_time_offset: Duration::minutes(
                (coord.x * Duration::days(1).num_minutes() as u32 + coord.y * timestep) as i64,
            ),
            duration: Duration::minutes(timestep as i64),
            org: org.to_string(),
        });
    } else if color >= color_objects.len() as u8 {
        panic!("Color was out of range: {}", color);
    } else {
        return Some(Reservation {
            start_time: start_datetime,
            objects: vec![TEObject::new(
                color_objects.get(color as usize).unwrap().as_str(),
            )],
            fields: vec![],
            start_time_offset: Duration::minutes(
                (coord.x * Duration::days(1).num_minutes() as u32 + coord.y * timestep) as i64,
            ),
            duration: Duration::minutes(timestep as i64),
            org: org.to_string(),
        });
    }
}
