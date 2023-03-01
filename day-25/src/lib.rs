fn to_snafu_char(decimal: i64) -> String {
    match decimal {
        0 => String::from("0"),
        1 => String::from("1"),
        2 => String::from("2"),
        3 => String::from("="),
        4 => String::from("-"),
        _ => panic!(
            "{decimal} number not able to be converted to a snafu char. must be between 0 and 4"
        ),
    }
}

fn from_snafu_char(character: char) -> i64 {
    match character {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '=' => -2,
        '-' => -1,
        _ => panic!("{character} is not a valid snafu character"),
    }
}

fn snafu_to_decimal(value: &str) -> i64 {
    let mut result = 0;
    for (index, character) in value.chars().rev().enumerate() {
        result += from_snafu_char(character) * 5_i64.pow((index as i64).try_into().unwrap())
    }
    result
}

fn dec_to_snafu(number: i64) -> String {
    let mut result = String::from("");
    let mut current = number;

    while current > 0 {
        let rem = current % 5;
        current /= 5;
        result += &to_snafu_char(rem);
        if rem > 2 {
            current += 1;
        }
    }

    result.chars().rev().collect()
}

pub fn part_one(input: &str) -> String {
    dec_to_snafu(input.lines().map(snafu_to_decimal).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_utilities() {
        assert_eq!(10, snafu_to_decimal("20"));
        assert_eq!(314159265, snafu_to_decimal("1121-1110-1=0"));
        assert_eq!("20", dec_to_snafu(10));
        assert_eq!("1=0", dec_to_snafu(15));
        assert_eq!("1121-1110-1=0", dec_to_snafu(314159265));
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!("2=-1=0", part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!("2-0-0=1-0=2====20=-2", part_one(INPUT));
    }
}
