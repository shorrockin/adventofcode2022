use std::collections::HashSet;
use std::iter::FromIterator;

fn priority(c: &char) -> u32 {
    // 0-9 reserved for numbers, offset by 9 to account for that
    let base = c.to_digit(36).unwrap() - 9;
    if c.is_uppercase() { base + 26 } else { base }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let set1: HashSet<_> = HashSet::from_iter(first.chars());
            let set2: HashSet<_> = HashSet::from_iter(second.chars());

            set1.intersection(&set2).map(|c| priority(c)).sum::<u32>()
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let set1: HashSet<_> = HashSet::from_iter(chunk[0].chars());
            let set2: HashSet<_> = HashSet::from_iter(chunk[1].chars());
            let set3: HashSet<_> = HashSet::from_iter(chunk[2].chars());

            set1.intersection(&set2)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&set3)
                .map(|c| priority(c))
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use indoc::indoc;

    static EXAMPLE: &str =
        indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    fn read_input_file() -> String {
        fs::read_to_string("input.txt").expect("oops - file could not be read")
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(157, part_one(EXAMPLE));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(8139, part_one(&read_input_file()));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(70, part_two(EXAMPLE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2668, part_two(&read_input_file()));
    }
}
