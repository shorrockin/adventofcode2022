pub mod utils;

use crate::utils::grid::directions::{EAST, SOUTH, WEST};
use crate::utils::grid::{BasicPoint, Coordinate, Direction, Grid};

const STARTING_GRID: &str = "-------";
const LOOP_DETECTOR_WARMUP_SIZE: usize = 200;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
enum Shape {
    HorizontalLine,
    Plus,
    RightAngle,
    VerticalLine,
    Square,
}
impl Shape {
    const HORIZONTAL_LINE_COORDS: [Direction; 4] = [
        Direction(0, 0),
        Direction(1, 0),
        Direction(2, 0),
        Direction(3, 0),
    ];

    const PLUS_LINE_COORDS: [Direction; 5] = [
        Direction(1, 0),
        Direction(0, -1),
        Direction(1, -1),
        Direction(2, -1),
        Direction(1, -2),
    ];

    const RIGHT_ANGLE_LINE_COORDS: [Direction; 5] = [
        Direction(0, 0),
        Direction(1, 0),
        Direction(2, 0),
        Direction(2, -1),
        Direction(2, -2),
    ];

    const VERTICAL_LINE_COORDS: [Direction; 4] = [
        Direction(0, 0),
        Direction(0, -1),
        Direction(0, -2),
        Direction(0, -3),
    ];

    const SQUARE_COORDS: [Direction; 4] = [
        Direction(0, 0),
        Direction(1, 0),
        Direction(0, -1),
        Direction(1, -1),
    ];

    fn next(&self) -> Shape {
        match self {
            Self::HorizontalLine => Self::Plus,
            Self::Plus => Self::RightAngle,
            Self::RightAngle => Self::VerticalLine,
            Self::VerticalLine => Self::Square,
            Self::Square => Self::HorizontalLine,
        }
    }

    // returns a vec of coordinates for this shape at the specified position.
    // shapes are defined as a list of coordinates relative to their most south
    // west coordinate.
    fn instance(self, at: &Coordinate) -> ShapeInstance {
        let coord_iter = match self {
            Self::HorizontalLine => Self::HORIZONTAL_LINE_COORDS.iter(),
            Self::Plus => Self::PLUS_LINE_COORDS.iter(),
            Self::RightAngle => Self::RIGHT_ANGLE_LINE_COORDS.iter(),
            Self::VerticalLine => Self::VERTICAL_LINE_COORDS.iter(),
            Self::Square => Self::SQUARE_COORDS.iter(),
        };
        ShapeInstance(coord_iter.map(|c| *at + *c).collect(), self)
    }
}

struct ShapeInstance(Vec<Coordinate>, Shape);
impl ShapeInstance {
    fn out_of_bounds(&self) -> bool {
        self.0
            .iter()
            .any(|coordinate| coordinate.0 < 0 || coordinate.0 >= 7)
    }

    fn intersects(&self, grid: &Grid<BasicPoint>) -> bool {
        self.0
            .iter()
            .any(|coordinate| grid.at(coordinate).is_some())
    }

    fn add_move(&self, movement: &Direction) -> ShapeInstance {
        let coords: Vec<_> = self
            .0
            .iter()
            .map(|coordinate| *coordinate + *movement)
            .collect();
        ShapeInstance(coords, self.1)
    }

    fn next_shape(&self) -> Shape {
        self.1.next()
    }
}

#[derive(Debug)]
struct MoveInstructions {
    time: usize,
    jets: Vec<Direction>,
}
impl MoveInstructions {
    fn new(input: &str) -> MoveInstructions {
        let jets: Vec<_> = input
            .chars()
            .map(|c| match c {
                '<' => WEST,
                '>' => EAST,
                _ => panic!("unexpected character: {}", c),
            })
            .collect();
        MoveInstructions { time: 0, jets }
    }

    fn next(&mut self) -> &Direction {
        self.time += 1;
        match self.time % 2 {
            1 => self
                .jets
                .get(self.index())
                .expect("expected to have jet at index"),
            _ => &SOUTH,
        }
    }

    fn index(&self) -> usize {
        (self.time / 2) % self.jets.len()
    }

    fn reset(&mut self) {
        if self.time % 2 == 1 {
            self.time += 1;
        }
    }
}

#[derive(Debug)]
struct LoopDetector {
    entries: Vec<LoopCacheEntry>,
    recordings: usize,
}
impl LoopDetector {
    fn new() -> LoopDetector {
        LoopDetector {
            entries: vec![],
            recordings: 0,
        }
    }

    fn record_rock(
        &mut self,
        shape: Shape,
        total_rocks: i64,
        movement_index: usize,
        height: i32,
    ) -> Option<(i64, i32)> {
        if shape == Shape::HorizontalLine {
            self.recordings += 1;
            if self.recordings > LOOP_DETECTOR_WARMUP_SIZE {
                self.entries.push(LoopCacheEntry {
                    total_rocks,
                    movement_index,
                    height,
                });

                // check to see if we've already record an entry for this shape at
                // this move index, if we recorded more than 3 then check for
                // the loop
                let same_movements: Vec<_> = self
                    .entries
                    .iter()
                    .filter(|entry| entry.movement_index == movement_index)
                    .collect();
                if same_movements.len() >= 2 {
                    let first = same_movements.get(0).unwrap();
                    let second = same_movements.get(1).unwrap();
                    return Some((
                        second.total_rocks - first.total_rocks,
                        second.height - first.height,
                    ));
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct LoopCacheEntry {
    total_rocks: i64,
    movement_index: usize,
    height: i32,
}

pub fn part_one(input: &str) -> i64 {
    simulate(input, 2022)
}

pub fn part_two(input: &str) -> i64 {
    simulate(input, 1000000000000)
}

fn simulate(input: &str, max_rocks: i64) -> i64 {
    let mut directions = MoveInstructions::new(input);
    let mut grid = Grid::from(STARTING_GRID, BasicPoint::new);
    let mut rock_counter = 0;
    let mut current_shape = Shape::HorizontalLine.instance(&starting_coordinate(&grid));
    let mut loop_detector = LoopDetector::new();
    let mut loop_adjusted_height: i64 = 0;

    loop {
        if rock_counter >= max_rocks {
            break;
        }

        let next_movement = directions.next();
        let next_shape = current_shape.add_move(next_movement);

        // only occurs on left/right movement, indicates that nothing happened
        // this iteration.
        if next_shape.out_of_bounds() {
            continue;
        }

        // if we intersect with any existing grid point, then we also can't
        // move, in this case the shape becomes frozen and we move onto our next
        // shape.
        if next_shape.intersects(&grid) {
            // only halt when we intersect as a result of falling, not being
            // pushed to the left/right
            if *next_movement != SOUTH {
                continue;
            }

            current_shape.0.iter().for_each(|coordinate| {
                grid.insert(BasicPoint::new(*coordinate, '#'));
            });
            rock_counter += 1;

            if let Some((every_rocks, increased_amount)) = loop_detector.record_rock(
                current_shape.1,
                rock_counter,
                directions.index(),
                grid.min_height.abs(),
            ) {
                // at this point we know we can fast forward based on the loop
                // metrics returned to us, ignore if we've already done so
                if loop_adjusted_height == 0 {
                    let rocks_remaining = max_rocks - rock_counter;
                    let fast_forward_loops = rocks_remaining / every_rocks;

                    rock_counter += fast_forward_loops * every_rocks;
                    loop_adjusted_height = (increased_amount as i64) * fast_forward_loops;
                }
            }

            current_shape = current_shape
                .next_shape()
                .instance(&starting_coordinate(&grid));

            directions.reset(); // ensure our next direction is left/right
            continue;
        }

        // otherwise the shape movement is fine and we can continue until the
        // we intersect
        current_shape = next_shape;
    }
    (grid.min_height.abs() as i64) + loop_adjusted_height
}

fn starting_coordinate(grid: &Grid<BasicPoint>) -> Coordinate {
    Coordinate(2, grid.min_height - 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_move_directions() {
        let mut instructions = MoveInstructions::new("<>><");
        assert_eq!(&WEST, instructions.next());
        assert_eq!(&SOUTH, instructions.next());
        assert_eq!(&EAST, instructions.next());
        assert_eq!(&SOUTH, instructions.next());
        assert_eq!(&EAST, instructions.next());
        assert_eq!(&SOUTH, instructions.next());
        assert_eq!(&WEST, instructions.next());
        assert_eq!(&SOUTH, instructions.next());
        assert_eq!(&WEST, instructions.next());
        assert_eq!(&SOUTH, instructions.next());
        assert_eq!(&EAST, instructions.next());
        assert_eq!(&SOUTH, instructions.next());
        assert_eq!(12, instructions.time);
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(3068, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(3206, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(1_514_285_714_288, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1_602_881_844_347, part_two(INPUT));
    }
}
