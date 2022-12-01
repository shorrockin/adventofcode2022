use std::fs;
use itertools::Itertools;

fn main() {
    println!("Part 1 Result: {:?}", part1(&read_input_file()));
    println!("Part 2 Result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Oops - file could not be read")
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {line.parse::<usize>().unwrap()})
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .tuple_windows()
        .filter(|(first, second)| first < second)
        .count()
}

fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .tuple_windows()
        .map(|(first, second, third)| first + second + third)
        .tuple_windows()
        .filter(|(first, second)| first < second)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*; 
    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    fn test_part_1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_1() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1559);
    }

    #[test]
    fn test_part_2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 5);
    } 

    #[test]
    fn test_part_2() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1600);
    }
}

