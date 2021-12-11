pub fn sonar_sweep<I>(readings: I) -> u64
where
    I: IntoIterator<Item = i64>,
{
    struct SweepStatus {
        last_measurement: Option<i64>,
        raised_count: u64,
    }

    readings.into_iter().fold(
        SweepStatus {last_measurement: None, raised_count: 0},
        |analysis, current_reading|
            SweepStatus {
                last_measurement: Some(current_reading),
                raised_count: analysis.last_measurement.map_or(
                    analysis.raised_count,
                    |last_reading|
                        if current_reading > last_reading { analysis.raised_count + 1 } else { analysis.raised_count }
                )
            }
    ).raised_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reports_zero_for_empty_list() {
        let count = sonar_sweep([]);

        assert_eq!(0, count);
    }

    #[test]
    fn it_correctly_counts_the_example_case() {
        let count = sonar_sweep([
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ]);

        assert_eq!(7, count);
    }
}
