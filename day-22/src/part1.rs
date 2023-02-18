use crate::movement::Movement;
use crate::utils::grid::{
    directions::{EAST, NORTH, SOUTH, WEST},
    Coordinate, Direction, Grid, Point,
};
use std::any::Any;

#[derive(Debug)]
struct Map {
    grid: Grid<Tile>,
    movement: Vec<Movement>,
    state: State,
}
impl Map {
    fn new(input: &str) -> Map {
        let (grid_str, movement_str) = input.split_once("\n\n").unwrap();
        let grid = Grid::from(grid_str, Tile::new);
        let movement = Movement::from(movement_str);

        let starting_point = grid.scan(Coordinate(grid.min_width, grid.min_height), EAST);
        let state = State {
            facing: EAST,
            position: starting_point.unwrap().coord,
        };

        Map {
            grid,
            movement,
            state,
        }
    }

    fn apply_movement(&mut self) {
        for movement in &self.movement {
            match movement {
                Movement::TurnLeft => self.state.facing = self.state.facing.turn_left(),
                Movement::TurnRight => self.state.facing = self.state.facing.turn_right(),
                Movement::Forward(distance) => {
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
pub struct Tile {
    coord: Coordinate,
    tile_type: TileType,
}
impl Tile {
    pub fn new(coord: Coordinate, character: char) -> Tile {
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

    pub fn is_wall(&self) -> bool {
        self.tile_type == TileType::Wall
    }

    pub fn is_void(&self) -> bool {
        self.tile_type == TileType::Void
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

#[derive(Debug)]
struct State {
    facing: Direction,
    position: Coordinate,
}

pub fn run(input: &str) -> i32 {
    let mut map = Map::new(input);
    map.apply_movement();
    map.password()
}
