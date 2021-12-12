use std::fs::File;
use crate::day2::navigate;

mod day2;

fn main() {
    let instructions = utilities::readers::read_strings(
        File::open(format!("{}/inputs/day2", env!("CARGO_MANIFEST_DIR"))).unwrap()
    ).unwrap();

    let result = navigate(&instructions);
    println!("Resulting position: ({}, {}) - multiplied: {}", result.depth, result.horizontal_position, result.depth * result.horizontal_position)
}
