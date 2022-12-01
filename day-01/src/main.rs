use std::fs;
use itertools::Itertools;

fn main() {
    println!("Part 1 Result: {:?}", part1(&read_input_file()));
    println!("Part 2 Result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("oops - file could not be read")
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim() // clean up test input
        .split("\n\n")
        .map(|group| group 
             .split("\n")
             .map(|value| value.parse::<usize>().unwrap())
             .sum())
        .collect()
}

fn part1(input: &str) -> usize {
    *parse(input)
        .iter()
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*; 
    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000    
    "};

    #[test]
    fn test_part_1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part_1() {
        let result = part1(&read_input_file());
        assert_eq!(result, 71924);
    }

    #[test]
    fn test_part_2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 45000);
    } 

    #[test]
    fn test_part_2() {
        let result = part2(&read_input_file());
        assert_eq!(result, 210406);
    }
}

