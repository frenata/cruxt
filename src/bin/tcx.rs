use std::env;
use std::fs;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_xml_rs::de::{Deserializer};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct TrainingCenterDatabase {
    activities: Activities
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Activities {
    #[serde(rename = "$value")]
    activities: Vec<Activity>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Activity {
    id: String,
    lap: Lap,
    sport: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Lap {
    start_time: DateTime<Utc>,
    total_time_seconds: f32,
    distance_meters: f32,
    maximum_speed: f32,
    calories: u32,
    average_heart_rate_bpm: Option<HR>,
    maximum_heart_rate_bpm: Option<HR>,
    cadence: u32,
    intensity: String,
    trigger_method: String,
    track: Track,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Track {
    #[serde(rename = "$value")]
    points: Vec<TrackPoint>
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
    time: DateTime<Utc>,
    position: Option<Position>,
    altitude_meters: Option<f32>,
    distance_meters: f32,
    heart_rate_bpm: Option<HR>,
    sensor_state: Option<String>,
    cadence: Option<u32>,
    extensions: Option<Extensions>,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = fs::File::open(file_path).expect(&format!("Cannot open file {}", file_path));
    let reader = BufReader::new(file);
    let deserializer = &mut Deserializer::new_from_reader(reader);
    let result: Result<TrainingCenterDatabase, _> = serde_path_to_error::deserialize(deserializer);
    match result {
        Ok(_) => {
            println!("done");
        }
        Err(err) => {
            let path = err.path().to_string();
            panic!("Could not deserialize at: {}", path);
        }
    }
}
