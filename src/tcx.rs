use std::io;
use serde::{Deserialize, Serialize};
use serde_xml_rs::de::{Deserializer};
use serde_xml_rs::ser::{Serializer};
use serde_xml_rs::{from_reader, to_writer};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TrainingCenterDatabase {
    pub activities: Activities
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Activities {
    #[serde(rename = "$value")]
    pub activity: Vec<Activity>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Activity {
    pub id: String,
    pub lap: Lap,
    pub sport: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Lap {
    start_time: DateTime<Utc>,
    total_time_seconds: f32,
    distance_meters: f32,
    maximum_speed: f32,
    pub calories: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    average_heart_rate_bpm: Option<HR>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_heart_rate_bpm: Option<HR>,
    cadence: u32,
    intensity: String,
    trigger_method: String,
    track: Track,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Track {
    #[serde(rename = "$value")]
    pub trackpoint: Vec<TrackPoint>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct  Position {
    latitude_degrees: f64,
    longitude_degrees: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Extensions {
    #[serde(rename = "TPX")]
    tpx: TPX
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct TPX {
    watts: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct HR {
    value: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct TrackPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    altitude_meters: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distance_meters: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate_bpm: Option<HR>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensor_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cadence: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<Extensions>,
}

pub fn read(stream: &mut impl io::Read) -> TrainingCenterDatabase {
    //return from_reader(stream).unwrap();

    let deserializer = &mut Deserializer::new_from_reader(stream);
    let result: Result<TrainingCenterDatabase, _> = serde_path_to_error::deserialize(deserializer);
    match result {
        Ok(db) => {
            return db;
        }
        Err(err) => {
            let path = err.path().to_string();
            panic!("Could not deserialize at: {}", path);
        }
    }
}

pub fn write(buf: &mut impl io::Write, db: &mut TrainingCenterDatabase) {
    //to_writer(buf, &db).unwrap();

    let serializer = &mut Serializer::new(buf);
    let mut track = serde_path_to_error::Track::new();
    let error_serializer = serde_path_to_error::Serializer::new(serializer, &mut track);
    //let result: Result<TrainingCenterDatabase, _> = serde_path_to_error::serialize(serializer);
    match db.serialize(error_serializer) {
        Ok(_) => {
            println!("wrote to file")
        }
        Err(_) => {
            let path = track.path().to_string();
            panic!("Could not serialize at: {}", path);
        }
    }
}
