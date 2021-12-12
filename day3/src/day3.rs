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