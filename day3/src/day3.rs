use std::ops::Index;

struct BitCounter {
    counts_per_position: Vec<u64>,
    total: u64
}

impl Index<u64> for BitCounter {
    type Output = u64;

    fn index(&self, index: u64) -> &Self::Output {
        return &self.counts_per_position[index as usize];
    }
}

impl BitCounter {
    fn add_binary_string(&self, binary: &String) -> Self {
        let mut new_counts: Vec<u64> = Vec::with_capacity(self.counts_per_position.len());

        for character in binary.char_indices() {
            new_counts.push(
                self.counts_per_position.get(character.0)
                    .unwrap_or(&0)
                    + if character.1 == '0' { 0 } else { 1 });
        }

        return BitCounter {
            total: self.total + 1,
            counts_per_position: new_counts,
        }
    }
}

pub struct PowerConsumption {
    pub gamma_rate: u64,
    pub epsilon_rate: u64,
}

impl From<&BitCounter> for PowerConsumption {
    fn from(counter: &BitCounter) -> Self {
        let mut gamma_rate = String::new();
        let mut epsilon_rate = String::new();

        for bit_count in counter.counts_per_position.iter() {
            gamma_rate.push_str( if bit_count > &(counter.total / 2) { "1" } else { "0" });
            epsilon_rate.push_str(if bit_count > &(counter.total / 2) { "0" } else { "1"});
        }

        return PowerConsumption{
            gamma_rate: u64::from_str_radix(gamma_rate.as_str(), 2).unwrap(),
            epsilon_rate: u64::from_str_radix(epsilon_rate.as_str(), 2).unwrap(),
        };
    }
}

pub fn calculate_power_consumption<'a, I>(readings: I) -> PowerConsumption
where
    I: IntoIterator<Item=&'a String>
{
    let bit_counter = readings.into_iter()
        .fold(
            BitCounter{
                total: 0,
                counts_per_position: Vec::with_capacity(8)
            },
            |counter, reading| counter.add_binary_string(reading)
        );

    return PowerConsumption::from(&bit_counter);
}

pub struct LifeSupportRatings {
    pub oxygen_rating: u64,
    pub co2_rating: u64,
}

enum BitCriteriaType {
    MostCommon,
    LeastCommon
}

fn get_most_common_bit(readings: &Vec<&String>, column: usize) -> bool
{
    let threshold = readings.len()/2;
    let ones_count = readings.iter()
        .fold(
            0,
            |sum, reading|
                reading.chars().nth(column).map_or(sum,
                   |bit| if bit == '1' { sum + 1 } else { sum }
                )
        );

    return if readings.len() % 2 == 0 { ones_count >= threshold } else { ones_count > threshold };
}

fn calculate_rating(readings: &Vec<&String>, filter_on: BitCriteriaType) -> u64
{
    let mut filtered = readings.clone();
    let mut current_column = 0 as usize;

    while filtered.len() > 1 {
        let most_common_bit = get_most_common_bit(
            &filtered,
            current_column
        );

        filtered = filtered.into_iter()
            .filter(|reading| {
                let char = reading.chars().nth(current_column).unwrap();
                let is_most_common = char == (if most_common_bit { '1' } else { '0' });

                return match filter_on {
                    BitCriteriaType::MostCommon => is_most_common,
                    BitCriteriaType::LeastCommon => !is_most_common,
                }
            })
            .collect();

        current_column += 1;
    }

    return filtered.first().map_or(
        0,
        |reading| u64::from_str_radix(reading.as_str(), 2).unwrap_or(0)
    );
}

pub fn calculate_life_support<'a, I>(readings: I) -> LifeSupportRatings
where
    I: IntoIterator<Item=&'a String>
{
    let readings_vector: Vec<&String> = readings.into_iter().collect();

    return LifeSupportRatings {
        oxygen_rating: calculate_rating(
            &readings_vector,
            BitCriteriaType::MostCommon
        ),
        co2_rating: calculate_rating(
            &readings_vector,
            BitCriteriaType::LeastCommon
        ),
    }
}