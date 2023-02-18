use crate::instructions;
use crate::instructions::Facing;
use crate::movement::Movement;
use crate::part1::Tile;
use crate::utils::grid;
use crate::utils::grid::Grid;
use std::collections::HashMap;

// main entry point for running part 2, takes in the input string along with
// instructions on how to interpret the input, saves us from having to write a
// generic solver.
pub fn run(input: &str, instructions: instructions::Cube) -> i32 {
    let (grid_str, instructions_str) = input.split_once("\n\n").unwrap();
    let movement = Movement::from(instructions_str);
    let grid = Grid::from(grid_str, Tile::new);
    let cube = Cube::from(grid, instructions);
    let start_position = Coordinate(Facing::Top, 0, 0, 0);
    let direction = Direction(Facing::Top, 1, 0, 0);

    dbg!(cube.0.get(&Coordinate(Facing::Top, 3, 0, 0)));
    dbg!(cube.0.get(&Coordinate(Facing::Top, 1, 0, 1)));
    dbg!(cube.0.get(&Coordinate(Facing::Top, 0, 0, 2)));
    dbg!(cube.0.get(&Coordinate(Facing::Top, 0, 0, 2)));
    999
}

#[derive(Debug)]
struct Direction(Facing, i32, i32, i32);

#[derive(Eq, PartialEq, Hash, Debug)]
struct Coordinate(Facing, usize, usize, usize);

#[derive(Debug)]
struct Point(bool);
impl Point {
    pub fn is_wall(&self) -> bool {
        self.0
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
                        "tile should never be void when parsing instructions"
                    );

                    let translated = (side.translator)(rel_x, rel_y);
                    let replaced = coordinates.insert(
                        Coordinate(side.facing, translated.0, translated.1, translated.2),
                        Point(tile.is_wall()),
                    );

                    assert!(
                        replaced.is_none(), 
                        "replaced value in map, this should not occur. side={:?}, x={}/{}, y={}/{}, translated={:?}", side.facing, rel_x, abs_x, rel_y, abs_y, translated
                    );
                }
            }
        }
        Cube(coordinates)
    }
}
