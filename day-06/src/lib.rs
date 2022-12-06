use itertools::Itertools;

pub fn first_unique_index(source: &str, count: usize) -> usize {
    source
        .chars()
        .collect::<Vec<char>>()
        .windows(count)
        .map(|window| window.iter().unique().collect())
        .find(|value: &String| value.len() == count)
        .map(|value| source.find(&value).unwrap() + count)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input_file() -> String {
        fs::read_to_string("input.txt").expect("oops - file could not be read")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1282, first_unique_index(&read_input_file(), 4));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3513, first_unique_index(&read_input_file(), 14));
    }
}
