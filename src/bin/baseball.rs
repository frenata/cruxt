use std::env;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PlateAppearance {
    #[serde(rename = "$value")]
    events: Vec<Event>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Event {
    Pitch(Pitch),
    Runner(Runner),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Pitch {
    speed: u32,
    r#type: PitchType,
    outcome: PitchOutcome,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PitchType { FourSeam, TwoSeam, Changeup, Cutter, Curve, Slider, Knuckle, Pitchout }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum PitchOutcome { Ball, Strike, Hit }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Runner {
    from: Base, to: Option<Base>, outcome: RunnerOutcome,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Base { First, Second, Third, Home }
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum RunnerOutcome { Steal, Caught, PickOff }

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let document = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let plate_appearance: PlateAppearance = from_str(&document).unwrap();
    assert_eq!(plate_appearance.events[0], Event::Pitch(Pitch { speed: 95, r#type: PitchType::FourSeam, outcome: PitchOutcome::Ball }));
}
