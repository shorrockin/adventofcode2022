pub mod utils;
use crate::utils::grid::{Coordinate, Direction, Grid, Point};

#[derive(Debug, PartialEq)]
enum Mode {
    PartOne,
    PartTwo,
}

pub fn part_one(input: &str) -> i32 {
    count_iterations(input, Mode::PartOne) - 1
}

pub fn part_two(input: &str) -> i32 {
    count_iterations(input, Mode::PartTwo)
}

fn count_iterations(input: &str, mode: Mode) -> i32 {
    let mut grid = parse(input);
    let origin = Coordinate(500, 0);
    let max_height = grid.max_height; // preserve as this will change
    let mut iterations = 0;

    loop {
        iterations += 1;
        match next_available_coordinate(&grid, origin, max_height, &mode) {
            Some(coord) => grid.insert(Block::new(coord, Material::Sand)),
            None => break,
        }
    }

    iterations
}

static DOWN: Direction = Direction(0, 1);
static DOWN_LEFT: Direction = Direction(-1, 1);
static DOWN_RIGHT: Direction = Direction(1, 1);

fn next_available_coordinate(
    grid: &Grid<Block>,
    origin: Coordinate,
    max_height: i32,
    mode: &Mode,
) -> Option<Coordinate> {
    let mut pos = origin;
    let floor = Block::new(Coordinate(-1, -1), Material::Rock);
    loop {
        let mut under = grid.at(pos + DOWN);
        let mut under_left = grid.at(pos + DOWN_LEFT);
        let mut under_right = grid.at(pos + DOWN_RIGHT);

        match mode {
            Mode::PartOne => {
                if pos.0 > grid.max_width || pos.1 > grid.max_height {
                    return None;
                }
            }
            Mode::PartTwo => {
                if pos.eq(&origin)
                    && under.is_some()
                    && under_left.is_some()
                    && under_right.is_some()
                {
                    return None;
                } else if pos.1 == max_height + 1 {
                    // in this scenario we just want everything to be Some()
                    // below, but the actual values don't really matter.
                    under = Some(&floor);
                    under_left = Some(&floor);
                    under_right = Some(&floor);
                }
            }
        }

        match (under, under_left, under_right) {
            (Some(_), Some(_), Some(_)) => break Some(pos),
            (None, _, _) => pos = pos + DOWN,
            (Some(_), None, _) => pos = pos + DOWN_LEFT,
            (Some(_), Some(_), None) => pos = pos + DOWN_RIGHT,
        }
    }
}

fn parse(input: &str) -> Grid<Block> {
    let mut grid: Grid<Block> = Grid::new();
    input
        .lines()
        .map(|line| line.split(" -> ").collect::<Vec<_>>())
        .map(|range_values| {
            range_values
                .iter()
                .map(|value| Coordinate::from(value))
                .collect::<Vec<_>>()
        })
        .for_each(|range| {
            range.windows(2).for_each(|window| {
                let start = window.get(0).unwrap();
                let end = window.get(1).unwrap();

                start
                    .to(*end)
                    .for_each(|coord| grid.insert(Block::new(coord, Material::Rock)));
            });
        });

    grid
}

#[derive(Debug)]
enum Material {
    Rock,
    Sand,
}

#[derive(Debug)]
struct Block {
    coord: Coordinate,
    material: Material,
}

impl Point for Block {
    fn symbol(&self) -> &str {
        match self.material {
            Material::Rock => "🪨",
            Material::Sand => "🪵",
        }
    }

    fn coord(&self) -> Coordinate {
        self.coord
    }
}

impl Block {
    fn new(coord: Coordinate, material: Material) -> Block {
        Block { coord, material }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(24, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(888, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(93, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(26461, part_two(INPUT));
    }
}
