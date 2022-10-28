use std::env;
use std::fs;
use std::io::{BufReader, BufWriter};

extern crate tcx;

fn parse(file_path: &String) -> tcx::TrainingCenterDatabase {
    let file = fs::File::open(file_path).expect(&format!("Cannot open file {}", file_path));
    let mut reader = BufReader::new(file);
    let result = tcx::read(&mut reader);
    println!("Calories burned: {}", result.activities.activity[0].lap.calories);
    return result;
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let mut _dbs: Vec<tcx::TrainingCenterDatabase> = args[1..]
        .iter()
        .map(parse)
        .collect();
    
    let file = fs::File::create("out.tcx").expect(&format!("Cannot open file {}", "out.tcx"));
    let mut writer = BufWriter::new(file);
    tcx::write(&mut writer, &mut _dbs[0]);
}
