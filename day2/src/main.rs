use std::fs::File;
use crate::day2::{better_navigate, navigate};

mod day2;

fn main() {
    let instructions = utilities::readers::read_strings(
        File::open(format!("{}/inputs/day2", env!("CARGO_MANIFEST_DIR"))).unwrap()
    ).unwrap();

    let result = navigate(&instructions);
    println!(
        "Resulting position: ({}, {}) - multiplied: {}",
        result.depth,
        result.horizontal_position,
        result.depth * result.horizontal_position
    );

    let better_result = better_navigate(&instructions);
    println!(
        "Better resulting position: ({}, {}) - multiplied: {}",
        better_result.position.depth,
        better_result.position.horizontal_position,
        better_result.position.depth * better_result.position.horizontal_position
    );
}
