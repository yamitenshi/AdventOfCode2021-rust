pub fn sonar_sweep<'a, I>(readings: I) -> u64
where
    I: IntoIterator<Item = &'a i64>,
{
    struct SweepStatus<'b> {
        last_measurement: Option<&'b i64>,
        raised_count: u64,
    }

    return readings.into_iter().fold(
        SweepStatus {last_measurement: None, raised_count: 0},
        |analysis, current_reading|
            SweepStatus {
                last_measurement: Some(current_reading),
                raised_count: analysis.last_measurement.map_or(
                    analysis.raised_count,
                    |last_reading|
                        if current_reading > last_reading { analysis.raised_count + 1 }
                        else { analysis.raised_count }
                )
            }
    ).raised_count;
}

#[cfg(test)]
mod sonar_sweep_tests {
    use super::sonar_sweep;

    #[test]
    fn it_reports_zero_for_empty_list() {
        let count = sonar_sweep(&[]);

        assert_eq!(0, count);
    }

    #[test]
    fn it_correctly_counts_the_example_case() {
        let count = sonar_sweep(&[
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

type SlidingWindow = [Option<i64>; 3];

fn sum(window: SlidingWindow) -> Option<i64> {
    if window.contains(&None) {
        return None;
    }

    return window.iter()
        .map(|measurement| measurement.unwrap())
        .reduce(|sum, measurement| sum + measurement);
}

fn push(window: SlidingWindow, value: i64) -> SlidingWindow {
    [window[1], window[2], Some(value)]
}

fn is_raised(left: Option<i64>, right: Option<i64>) -> bool {
    if left.is_none() || right.is_none() {
        return false;
    }

    return left.unwrap() > right.unwrap();
}

#[cfg(test)]
mod sliding_window_tests {
    use super::sum;
    use super::push;
    use super::is_raised;

    #[test]
    fn it_does_not_consider_none_higher_than_some() {
        assert!(!is_raised(None, Some(-500)));
    }

    #[test]
    fn it_does_not_consider_some_higher_than_none() {
        assert!(!is_raised(Some(500), None));
    }

    #[test]
    fn it_considers_higher_some_on_left_raised() {
        assert!(is_raised(Some(500), Some(499)));
    }

    #[test]
    fn it_considers_lower_some_on_left_not_raised() {
        assert!(!is_raised(Some(499), Some(500)));
    }

    #[test]
    fn it_sums_empty_window_to_none() {
        let sum = sum([None, None, None]);

        assert!(sum.is_none());
    }

    #[test]
    fn it_sums_partially_filled_window_to_none() {
        let sum = sum([None, None, Some(1)]);

        assert!(sum.is_none());
    }

    #[test]
    fn it_sums_fully_filled_window() {
        let sum = sum([Some(1), Some(2), Some(3)]);

        assert_eq!(Some(6), sum);
    }

    #[test]
    fn it_pushes_onto_empty_window() {
        let new = push([None, None, None], 1);

        assert_eq!([None, None, Some(1)], new);
    }

    #[test]
    fn it_pushes_onto_partially_filled_window() {
        let new = push([None, None, Some(1)], 2);

        assert_eq!([None, Some(1), Some(2)], new);
    }

    #[test]
    fn it_pushes_onto_filled_window_and_shifts_off_first_value() {
        let new = push([Some(1), Some(2), Some(3)], 4);

        assert_eq!([Some(2), Some(3), Some(4)], new);
    }
}

pub fn sliding_sweep<'a, I>(readings: I) -> u64
    where
        I: IntoIterator<Item = &'a i64>,
{
    struct SweepStatus {
        last_measurements: SlidingWindow,
        raised_count: u64,
    }

    return readings.into_iter().fold(
        SweepStatus {last_measurements: [None, None, None], raised_count: 0},
        |analysis, current_reading| {
            let new_window = push(analysis.last_measurements, *current_reading);

            SweepStatus {
                last_measurements: new_window,
                raised_count:
                    if is_raised(sum(new_window), sum(analysis.last_measurements)) { analysis.raised_count + 1 }
                    else { analysis.raised_count }
            }
        }
    ).raised_count;
}

#[cfg(test)]
mod sliding_sweep_tests {
    use super::sliding_sweep;

    #[test]
    fn it_reports_zero_for_empty_list() {
        let result = sliding_sweep(&[]);

        assert_eq!(0, result);
    }

    #[test]
    fn it_reports_zero_for_too_few_values() {
        let result = sliding_sweep(&[1, 2]);

        assert_eq!(0, result);
    }

    #[test]
    fn it_correctly_analyses_example_case() {
        let result = sliding_sweep(&[
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

        assert_eq!(5, result);
    }
}