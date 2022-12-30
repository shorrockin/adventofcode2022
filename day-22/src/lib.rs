#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
pub mod utils;

use std::any::Any;
use utils::grid::{
    directions::{EAST, NORTH, SOUTH, WEST},
    Coordinate, Direction, Grid, Point,
};

#[derive(Debug)]
struct Map {
    grid: Grid<Tile>,
    instructions: Vec<Instruction>,
    state: State,
}
impl Map {
    fn new(input: &str) -> Map {
        let (grid_str, instructions_str) = input.split_once("\n\n").unwrap();
        let grid = Grid::from(grid_str, Tile::new);
        let instructions = Instruction::from(instructions_str);

        let starting_point = grid.scan(Coordinate(grid.min_width, grid.min_height), EAST);
        let state = State {
            facing: EAST,
            position: starting_point.unwrap().coord,
        };

        Map {
            grid,
            instructions,
            state,
        }
    }

    fn apply_instructions(&mut self) {
        for instruction in &self.instructions {
            match instruction {
                Instruction::TurnLeft => self.state.facing = self.state.facing.turn_left(),
                Instruction::TurnRight => self.state.facing = self.state.facing.turn_right(),
                Instruction::Forward(distance) => {
                    for _ in 0..*distance {
                        let next_coordinate = self.state.position + self.state.facing;
                        match self.grid.at(&next_coordinate) {
                            Some(tile) => match tile.tile_type {
                                TileType::Wall => break,
                                TileType::Floor => self.state.position = next_coordinate,
                                TileType::Void => panic!("should not have void tiles"),
                            },
                            None => {
                                let scan_start = if self.state.facing == EAST {
                                    Coordinate(self.grid.min_width, self.state.position.1)
                                } else if self.state.facing == SOUTH {
                                    Coordinate(self.state.position.0, self.grid.min_height)
                                } else if self.state.facing == WEST {
                                    Coordinate(self.grid.max_width, self.state.position.1)
                                } else if self.state.facing == NORTH {
                                    Coordinate(self.state.position.0, self.grid.max_height)
                                } else {
                                    panic!("hit unexpected direction")
                                };
                                match self.grid.scan(scan_start, self.state.facing) {
                                    Some(point) => match point.tile_type {
                                        TileType::Wall => break,
                                        TileType::Floor => self.state.position = point.coord,
                                        TileType::Void => panic!("should not have void warp tiles"),
                                    },
                                    None => panic!(
                                        "could not find warp point from {} in direction of {}",
                                        scan_start, self.state.facing
                                    ),
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn password(&self) -> i32 {
        ((self.state.position.0 + 1) * 4)
            + ((self.state.position.1 + 1) * 1000)
            + if self.state.facing == EAST {
                0
            } else if self.state.facing == SOUTH {
                1
            } else if self.state.facing == WEST {
                2
            } else if self.state.facing == NORTH {
                3
            } else {
                panic!("unsupported facing for password")
            }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Tile {
    coord: Coordinate,
    tile_type: TileType,
}
impl Tile {
    fn new(coord: Coordinate, character: char) -> Tile {
        Tile {
            coord,
            tile_type: match character {
                '.' => TileType::Floor,
                '#' => TileType::Wall,
                ' ' => TileType::Void,
                _ => panic!("expected symbol: '{}'", character),
            },
        }
    }
}
impl Point for Tile {
    fn coord(&self) -> Coordinate {
        self.coord
    }

    fn symbol(&self) -> &str {
        match self.tile_type {
            TileType::Wall => "▨",
            TileType::Floor => ".",
            TileType::Void => panic!("void type not supported"),
        }
    }

    fn ignore(&self) -> bool {
        self.tile_type == TileType::Void
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TileType {
    Wall,
    Floor,
    Void,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Forward(usize),
    TurnRight,
    TurnLeft,
}
impl Instruction {
    fn from(line: &str) -> Vec<Instruction> {
        let mut out = vec![];
        let mut current_digit = "".to_string();
        for character in line.chars() {
            match character {
                'R' => {
                    out.push(Instruction::Forward(current_digit.parse().unwrap()));
                    current_digit = "".to_string();
                    out.push(Instruction::TurnRight);
                }
                'L' => {
                    out.push(Instruction::Forward(current_digit.parse().unwrap()));
                    current_digit = "".to_string();
                    out.push(Instruction::TurnLeft);
                }
                _ => {
                    current_digit.push(character);
                }
            }
        }

        out.push(Instruction::Forward(current_digit.parse().unwrap()));
        out
    }
}

#[derive(Debug)]
struct State {
    facing: Direction,
    position: Coordinate,
}

pub fn part_one(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.apply_instructions();
    map.password()
}

pub fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(6032, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1428, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        // assert_eq!(99, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(99, part_two(INPUT));
    }
}
