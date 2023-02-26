#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
pub mod utils;
use pathfinding::prelude::bfs;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    str::FromStr,
};
use utils::grid::{Coordinate, Direction, Grid, Point};

use crate::utils::grid::directions::{EAST, NORTH, SOUTH, WEST};

mod strings {
    pub const WIND_EAST: &str = ">";
    pub const WIND_WEST: &str = "<";
    pub const WIND_NORTH: &str = "^";
    pub const WIND_SOUTH: &str = "v";

    pub const WALL: &str = "#";
    pub const EMPTY: &str = ".";
}

// on a specific turn caches the occupied squares
struct WindCache {
    turn: usize,
    occupied: HashSet<Coordinate>,
}
impl WindCache {
    fn new(turn: usize, grid: &Grid<Position>) -> WindCache {
        let max_width = grid.max_width - 1; // -1 for walls
        let max_height = grid.max_height - 1; // -1 for walls
        let occupied = grid
            .points
            .values()
            .filter_map(|point| match &point.definition {
                Definition::Wind(dir) => {
                    let base = point.coord + Direction(-1, -1); // remove walls
                    let movement = Direction(dir.0 * turn as i32, dir.1 * turn as i32);
                    let dest = base + movement;
                    let bound_dest =
                        Coordinate(dest.0.rem_euclid(max_width), dest.1.rem_euclid(max_height));
                    Some(bound_dest + Direction(1, 1)) // add walls back in
                }
                _ => None,
            })
            .collect();
        WindCache { turn, occupied }
    }

    fn all(turns: usize, grid: &Grid<Position>) -> Vec<WindCache> {
        (0..turns).map(|turn| WindCache::new(turn, grid)).collect()
    }
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            strings::WIND_EAST => Ok(EAST),
            strings::WIND_SOUTH => Ok(SOUTH),
            strings::WIND_WEST => Ok(WEST),
            strings::WIND_NORTH => Ok(NORTH),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Definition {
    Wall,
    Wind(Direction),
    Empty,
}
impl Definition {
    fn is_wind(&self) -> bool {
        matches!(self, Definition::Wind(_))
    }

    fn is_wall(&self) -> bool {
        matches!(self, Definition::Wall)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    coord: Coordinate,
    definition: Definition,
}
impl Position {
    fn new(coord: Coordinate, symbol: char) -> Position {
        let definition = match symbol {
            '.' => Definition::Empty,
            '#' => Definition::Wall,
            _ => Definition::Wind(Direction::from_str(&symbol.to_string()).unwrap()),
        };
        Position { coord, definition }
    }
}
impl Point for Position {
    fn symbol(&self) -> String {
        match &self.definition {
            Definition::Wall => strings::WALL.to_string(),
            Definition::Wind(direction) => "W".to_string(),
            Definition::Empty => strings::EMPTY.to_string(),
        }
    }

    fn coord(&self) -> utils::grid::Coordinate {
        self.coord
    }
}

// composes the coordinate to include the turn
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct CoordinateAtTurn(Coordinate, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

fn shortest_path(
    grid: &Grid<Position>,
    start: Coordinate,
    finish: Coordinate,
    starting: usize,
) -> usize {
    let cycle_size = lcm(grid.max_width as usize - 1, grid.max_height as usize - 1);
    let turn_cache = WindCache::all(cycle_size, grid);
    let start_turn = CoordinateAtTurn(start, starting);

    bfs(
        &start_turn,
        |current| {
            let turn = current.1 + 1;
            let wind_cache = turn_cache.get(turn % turn_cache.len()).unwrap();
            [
                current.0 + NORTH,
                current.0 + SOUTH,
                current.0 + EAST,
                current.0 + WEST,
                current.0,
            ]
            .iter()
            .filter(|coordinate| grid.at(coordinate).is_some())
            .filter(|coordinate| !grid.at(coordinate).unwrap().definition.is_wall())
            .filter(|coordinate| !wind_cache.occupied.contains(coordinate))
            // .filter(|coordinate| !coordinate.eq(&&start))
            .map(|coordinate| CoordinateAtTurn(*coordinate, turn))
            .collect::<Vec<_>>()
        },
        |success| success.0.eq(&finish),
    )
    .unwrap()
    .len()
        - 1
}

fn lcm(left: usize, right: usize) -> usize {
    let max_val = max(left, right);
    let min_val = min(left, right);

    for i in 1..=min_val {
        let multiple = max_val * i;
        if multiple % min_val == 0 {
            return multiple;
        }
    }

    max_val * min_val
}

pub fn part_one(input: &str) -> usize {
    let grid = Grid::from(input, Position::new);
    shortest_path(
        &grid,
        Coordinate(1, 0),
        Coordinate(grid.max_width - 1, grid.max_height),
        0,
    )
}

pub fn part_two(input: &str) -> usize {
    let grid = Grid::from(input, Position::new);
    let start = Coordinate(1, 0);
    let finish = Coordinate(grid.max_width - 1, grid.max_height);

    let first = shortest_path(&grid, start, finish, 0);
    let second = shortest_path(&grid, finish, start, first);
    let third = shortest_path(&grid, start, finish, first + second);

    first + second + third
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_utilities() {
        assert_eq!(Direction::from_str("<"), Ok(Direction(-1, 0)));
        assert!(Direction::from_str("x").is_err());
        assert_eq!(lcm(6, 4), 12)
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(18, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(262, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(54, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(785, part_two(INPUT));
    }
}
