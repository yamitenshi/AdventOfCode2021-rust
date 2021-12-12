mod day3;

extern crate utilities;

use std::fs::File;

fn main() {
    let readings = utilities::readers::read_strings(
        File::open(format!("{}/inputs/day3", env!("CARGO_MANIFEST_DIR"))).unwrap()
    ).unwrap();

    let power_consumption = day3::calculate_power_consumption(&readings);

    println!(
        "Epsilon: {}, Gamma: {}, Power consumption: {}",
        power_consumption.gamma_rate,
        power_consumption.epsilon_rate,
        power_consumption.epsilon_rate * power_consumption.gamma_rate
    )
}
