use crate::utils::grid;
use crate::utils::grid::Grid;
use core::fmt;
use std::collections::HashMap;

use super::instructions::{Face, self};
use super::movement::Movement;
use super::part1::Tile;

// main entry point for running part 2, takes in the input string along with
// instructions on how to interpret the input, saves us from having to write a
// generic solver.
pub fn run(input: &str, instructions: instructions::Cube) -> i32 {
    let (grid_str, instructions_str) = input.split_once("\n\n").unwrap();
    let movements = Movement::from(instructions_str);
    let grid = Grid::from(grid_str, Tile::new);
    let cube = Cube::from(grid, instructions);

    let mut position = Coordinate::new(Face::Top, 0, 0, 0);
    let mut direction = Direction::new(Face::Top, 1, 0, 0);

    for movement in movements {
        match movement {
            Movement::Forward(amount) => {
                for _ in 0..amount {
                    let (next_position, next_direction) = cube.move_coordinate(&position, &direction);
                    if !cube.is_wall(&next_position) {
                        position = next_position;
                        direction = next_direction;
                    } else {
                        break
                    }
                }
            },
            Movement::TurnRight => {
                direction = direction.turn_right();
            },
            Movement::TurnLeft => {
                direction = direction.turn_left();
            },
        }
    }

    let final_position = cube.0.get(&position).unwrap();
    let facing_score = match (direction.face, direction.x, direction.y, direction.z) {
        (Face::Top, 1, 0, 0) => 0, // Right
        (Face::Top, -1, 0, 0) => 2, // Left
        (Face::Top, 0, 0, 1) => 3, // Up
        (Face::Top, 0, 0, -1) => 1, // Down

        (Face::Bottom, 1, 0, 0) => 0, // Right
        (Face::Bottom, -1, 0, 0) => 2, // Left
        (Face::Bottom, 0, 0, 1) => 3, // Up
        (Face::Bottom, 0, 0, -1) => 1, // Down        

        (Face::Left, 0, 1, 0) => 1, // Down
        (Face::Left, 0, -1, 0) => 3, // Up
        (Face::Left, 0, 0, 1) => 0, // Right
        (Face::Left, 0, 0, -1) => 2, // Left

        _ => panic!("could not infer facing score, condition not included: {:?}:{}/{}/{}", direction.face, direction.x, direction.y, direction.z)
    };
    (1000 * (final_position.original_coordinate.1 + 1)) + (4 * (final_position.original_coordinate.0 + 1)) + facing_score
}

#[derive(Debug, Copy, Clone)]
struct Direction {
    face: Face,
    x: i32,
    y: i32,
    z: i32,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Direction(face={:#?}, {}/{}/{})", self.face, self.x, self.y, self.z)
    }
}
impl Direction {
    pub fn new(face: Face, x: i32, y: i32, z: i32) -> Direction {
        Direction {face, x, y, z}
    }

    // applies a rotation on the face in the direction we're facing, returns the
    // new direction once the rotation has been applied.
    pub fn next_direction(&self) -> Direction {
        let next_face = self.face.rotate(self.x, self.y, self.z);
        match self.face {
            Face::Top => Direction::new(next_face, 0, 1, 0),
            Face::Bottom => Direction::new(next_face, 0, -1, 0),
            Face::Right => Direction::new(next_face, -1, 0, 0),
            Face::Left => Direction::new(next_face, 1, 0, 0),
            Face::Front => Direction::new(next_face, 0, 0, -1),
            Face::Back => Direction::new(next_face, 0, 0, 1),
        }
    }

    pub fn turn_right(&self) -> Direction {
        match (self.face, self.x.abs(), self.y.abs(), self.z.abs()) {
            (Face::Top, 1, 0, 0) => Direction::new(self.face, 0, 0, self.x),
            (Face::Top, 0, 0, 1) => Direction::new(self.face, -self.z, 0, 0),
            (Face::Bottom, 1, 0, 0) => Direction::new(self.face, 0, 0, -self.x),
            (Face::Bottom, 0, 0, 1) => Direction::new(self.face, self.z, 0, 0),

            (Face::Front, 1, 0, 0) => Direction::new(self.face, 0, self.x, 0),
            (Face::Front, 0, 1, 0) => Direction::new(self.face, -self.y, 0, 0),
            (Face::Back, 1, 0, 0) => Direction::new(self.face, 0, -self.x, 0),
            (Face::Back, 0, 1, 0) => Direction::new(self.face, self.y, 0, 0),

            (Face::Right, 0, 1, 0) => Direction::new(self.face, 0, 0, self.y),
            (Face::Right, 0, 0, 1) => Direction::new(self.face, 0, -self.z, 0),
            (Face::Left, 0, 1, 0) => Direction::new(self.face, 0, 0, -self.y),
            (Face::Left, 0, 0, 1) => Direction::new(self.face, 0, self.z, 0),
            _ => panic!("unresolved pattern for turn right: {:?}, {}/{}/{}", self.face, self.x, self.y, self.z),
        }
    }

    pub fn turn_left(&self) -> Direction {
        let right = self.turn_right();
        Direction::new(self.face, -right.x, -right.y, -right.z)
    }
}


#[derive(Eq, PartialEq, Hash, Debug)]
struct Coordinate {
    face: Face,
    x: i32,
    y: i32,
    z: i32,
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate(face={:#?}, {}/{}/{})", self.face, self.x, self.y, self.z)
    }
}
impl Coordinate {
    pub fn new(face: Face, x: i32, y: i32, z: i32) -> Coordinate {
        Coordinate {face, x, y, z}
    }
}

#[derive(Debug)]
struct Point {
    original_coordinate: (i32, i32),
    wall: bool
}
impl Point {
    pub fn new(x: i32, y:i32, wall: bool) -> Point {
        Point {
            original_coordinate: (x, y),
            wall,
        }
    }
}

#[derive(Debug)]
struct Cube(HashMap<Coordinate, Point>);
impl Cube {
    pub fn from(grid: Grid<Tile>, instructions: instructions::Cube) -> Cube {
        let mut coordinates = HashMap::new();
        let size = instructions.size;

        for side in instructions.sides {
            for rel_x in 0..size {
                for rel_y in 0..size {
                    let abs_x = (size * side.start.0) + rel_x;
                    let abs_y = (size * side.start.1) + rel_y;
                    let tile = grid
                        .at(&grid::Coordinate(abs_x as i32, abs_y as i32))
                        .expect("we should have a tile at this coordinate");

                    assert!(
                        !tile.is_void(),
                        "tile should never be void when parsing instructions x={}, y={}", rel_x, rel_y
                    );

                    let translated = (side.translator)(rel_x, rel_y);
                    let replaced = coordinates.insert(
                        Coordinate::new(side.face, translated.0, translated.1, translated.2),
                        Point::new(abs_x, abs_y, tile.is_wall()),
                    );

                    assert!(
                        replaced.is_none(), 
                        "replaced value in map, this should not occur. face={:?}, x={}/{}, y={}/{}, translated={:?}", side.face, rel_x, abs_x, rel_y, abs_y, translated
                    );
                }
            }
        }
        Cube(coordinates)
    }

    pub fn is_wall(&self, at: &Coordinate) -> bool {
        if let Some(point) = self.0.get(at) {
            point.wall
        } else {
            panic!("unable to determine wall coordinate, invalid position")
        }
    }

    pub fn move_coordinate(&self, from: &Coordinate, direction: &Direction) -> (Coordinate, Direction) {
        let potential_next = Coordinate::new(from.face, from.x + direction.x, from.y + direction.y, from.z + direction.z);

        // we have this coordinate, no change need to face we can just return
        // with no coordinate math applied
        if self.0.contains_key(&potential_next) {
            return (potential_next, *direction);
        }

        // otherwise we won't change coordinate, but we will change both our
        // face and the direction we are going.
        let next_direction = direction.next_direction();
        (Coordinate::new(next_direction.face, from.x, from.y, from.z), next_direction)
    }
}
