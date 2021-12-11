use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn read_ints(input: impl Read) -> Result<Vec<i64>,Error> {
    let br = BufReader::new(input);

    br.lines()
        .map(|line| line.unwrap())
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_an_empty_string_into_an_empty_vector() {
        let result = read_ints("".as_bytes());

        assert!(
            result.is_ok(),
            "Reading from empty file results in error state: {:?}",
            result.err().unwrap()
        );

        assert_eq!(
            result.ok().unwrap(),
            vec![]
        );
    }

    #[test]
    fn it_reads_a_single_line_into_an_integer() {
        let result = read_ints("123".as_bytes());

        assert!(
            result.is_ok(),
            "Reading from single line file results in error state: {:?}",
            result.err().unwrap()
        );

        assert_eq!(
            result.ok().unwrap(),
            vec![123]
        );
    }

    #[test]
    fn it_reads_multiple_lines_into_multiple_integers() {
        let content = "1\n\
            2\n\
            3"
            .as_bytes();

        let result = read_ints(content);

        assert!(
            result.is_ok(),
            "Reading from multiline file results in error state: {:?}",
            result.err().unwrap()
        );

        assert_eq!(
            result.ok().unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn it_skips_blank_lines() {
        let content = "1\n\
            \n\
            2\n\
            \n\
            \n\
            3"
            .as_bytes();

        let result = read_ints(content);

        assert!(
            result.is_ok(),
            "Reading from multiline file with blank lines results in error state: {:?}",
            result.err().unwrap()
        );

        assert_eq!(
            result.ok().unwrap(),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn it_errors_on_unparseable_values() {
        let content = "\
            1\n\
            three"
            .as_bytes();

        let result = read_ints(content);

        assert!(
            result.is_err(),
            "Reading from file with unparseable values did not result in an error value! Instead we got: {:?}",
            result.ok().unwrap()
        );
    }
}