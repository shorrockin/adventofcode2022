use itertools::Itertools;

pub fn first_unique_index(source: &str, count: usize) -> usize {
    let characters: Vec<char> = source.chars().collect();
    let first_occurrence: String = characters
        .windows(count)
        .map(|window| window.iter().unique().collect())
        .filter(|value: &String| value.len() == count)
        .next()
        .unwrap();

    source.find(&first_occurrence).unwrap() + count
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
