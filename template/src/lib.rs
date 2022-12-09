pub mod utils;

pub fn part_one(_input: &str) -> i32 {
    -1
}

pub fn part_two(_input: &str) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
    "};

    fn read_input_file() -> String {
        include_str!("../input.txt").to_string()
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(-1, part_one(EXAMPLE));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(-1, part_one(&read_input_file()));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(-1, part_two(EXAMPLE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(-1, part_two(&read_input_file()));
    }
}
