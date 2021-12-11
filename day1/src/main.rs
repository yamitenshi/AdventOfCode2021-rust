mod day1;

extern crate utilities;

use std::fs::File;
use crate::day1::sonar_sweep;

fn main() {
    let readings = utilities::readers::read_ints(
        File::open(format!("{}/inputs/day1", env!("CARGO_MANIFEST_DIR"))).unwrap()
    );

    println!("Sonar sweep: {}", sonar_sweep(readings.unwrap()));
}
