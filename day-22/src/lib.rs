#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
mod instructions;
mod movement;
mod part1;
mod part2;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(6032, part1::run(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1428, part1::run(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        use crate::instructions::*;

        let size = 4;
        let instructions = Cube::new(
            4,
            vec![
                Side::new(Facing::Top, 2, 0, |x, y| (x, 0, y)),
                Side::new(Facing::Front, 2, 1, |x, y| (x, y, 3)),
                Side::new(Facing::Left, 1, 1, |x, y| (0, y, x)),
                Side::new(Facing::Back, 0, 1, |x, y| (3 - x, y, 0)),
                Side::new(Facing::Bottom, 2, 2, |x, y| (x, 3, 3 - y)),
                Side::new(Facing::Right, 2, 2, |x, y| (3, 3 - x, 3 - y)),
            ],
        );

        // running in separate mod as i'm doing this weeks apart and don't want
        // to approach this with a fresh brain and not try to create a cohesive
        // solution for both parts.
        assert_eq!(5031, part2::run(EXAMPLE_INPUT, instructions));
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(99, part_two(INPUT));
    }
}
