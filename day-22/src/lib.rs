pub mod solution;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::solution::{instructions::*, part1, part2};

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
        let instructions = Cube::new(
            4,
            vec![
                Side::new(Face::Top, 2, 0, |x, y| (x, 0, y)),
                Side::new(Face::Front, 2, 1, |x, y| (x, y, 3)),
                Side::new(Face::Left, 1, 1, |x, y| (0, y, x)),
                Side::new(Face::Back, 0, 1, |x, y| (3 - x, y, 0)),
                Side::new(Face::Bottom, 2, 2, |x, y| (x, 3, 3 - y)),
                Side::new(Face::Right, 3, 2, |x, y| (3, 3 - x, 3 - y)),
            ],
        );

        // running in separate mod as i'm doing this weeks apart and don't want
        // to approach this with a fresh brain and not try to create a cohesive
        // solution for both parts.
        assert_eq!(5031, part2::run(EXAMPLE_INPUT, instructions));
    }

    #[test]
    fn test_part_two() {
        let instructions = Cube::new(
            50,
            vec![
                Side::new(Face::Top, 1, 0, |x, y| (x, 0, y)),
                Side::new(Face::Right, 2, 0, |x, y| (49, x, y)),
                Side::new(Face::Front, 1, 1, |x, y| (x, y, 49)),
                Side::new(Face::Bottom, 1, 2, |x, y| (x, 49, 49 - y)),
                Side::new(Face::Left, 0, 2, |x, y| (0, x, 49 - y)),
                Side::new(Face::Back, 0, 3, |x, y| (y, x, 0)),
            ],
        );

        assert_eq!(142380, part2::run(INPUT, instructions));
    }
}
