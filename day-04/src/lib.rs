use std::ops::Range;

pub fn part_one(input: &str) -> usize {
    parse(input, &full_overlap)
}

pub fn part_two(input: &str) -> usize {
    parse(input, &partial_overlap)
}

fn parse(input: &str, filter: &dyn Fn(&Range<u32>, &Range<u32>) -> bool) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(first, second)| (str_to_range(first), str_to_range(second)))
        .filter(|(first, second)| filter(first, second))
        .count()
}

fn str_to_range(range_string: &str) -> Range<u32> {
    let (from, to) = range_string.split_once('-').unwrap();
    from.parse().unwrap()..to.parse().unwrap()
}

fn partial_overlap(first: &Range<u32>, second: &Range<u32>) -> bool {
    (first.start >= second.start && first.start <= second.end)
        || (first.end >= second.start && first.end <= second.end)
        || (second.start >= first.start && second.start <= first.end)
        || (second.end >= first.start && second.end <= first.end)
}

fn full_overlap(first: &Range<u32>, second: &Range<u32>) -> bool {
    (first.start >= second.start && first.end <= second.end)
        || (second.start >= first.start && second.end <= first.end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::fs;

    static EXAMPLE: &str = indoc! {"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
    "};

    fn read_input_file() -> String {
        fs::read_to_string("input.txt").expect("oops - file could not be read")
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(2, part_one(EXAMPLE));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(511, part_one(&read_input_file()));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(4, part_two(EXAMPLE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(821, part_two(&read_input_file()));
    }
}
