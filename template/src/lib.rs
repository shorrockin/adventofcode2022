pub mod utils;

pub fn part_one(_input: &str) -> usize {
    0
}

pub fn part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(99, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        // assert_eq!(99, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        // assert_eq!(99, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(99, part_two(INPUT));
    }
}
