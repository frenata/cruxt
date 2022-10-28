use std::env;
use std::fs;
use std::io::BufReader;

extern crate tcx;

fn parse(file_path: &String) -> tcx::TrainingCenterDatabase {
    let file = fs::File::open(file_path).expect(&format!("Cannot open file {}", file_path));
    let mut reader = BufReader::new(file);
    let result = tcx::read(&mut reader);
    println!("Calories burned: {}", result.activities.list[0].lap.calories);
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let _dbs: Vec<tcx::TrainingCenterDatabase> = args[1..]
        .iter()
        .map(parse)
        .collect();
}
