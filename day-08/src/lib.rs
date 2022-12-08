pub mod utils;
use crate::utils::grid::directions::{EAST, NORTH, SOUTH, WEST};
use crate::utils::grid::{Grid, Point};
// use colored::Colorize;
use core::cmp::Ordering;
use std::collections::HashSet;

enum VisibilityRules {
    PartOne,
    PartTwo,
    None,
}

// simple model used as the element needed to house each point in our grid
#[derive(PartialEq, Eq, Hash, Debug)]
struct LandPlot {
    x: i32,
    y: i32,
    height: usize,
}

impl Point for LandPlot {
    fn coord(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn symbol(&self) -> String {
        self.height.to_string()
    }
}

impl LandPlot {
    fn new(x: i32, y: i32, symbol: char) -> LandPlot {
        LandPlot {
            x,
            y,
            height: symbol.to_digit(10).unwrap() as usize,
        }
    }
}

// implementation of an iteration which iterates over our grid construct in a
// specific direction. if visibility_range is true, only values which are
// visible will be returned by the iterator.
struct GridIterator<'a> {
    grid: &'a Grid<LandPlot>,
    next: (i32, i32),
    direction: (i32, i32),
    previous: Option<(i32, i32)>,
    visibility: VisibilityRules,
    max_height: usize,
}

impl GridIterator<'_> {
    fn new(
        grid: &Grid<LandPlot>,
        starting: (i32, i32),
        direction: (i32, i32),
        visibility: VisibilityRules,
    ) -> GridIterator {
        GridIterator {
            grid,
            next: starting,
            direction,
            previous: None,
            visibility,
            max_height: grid.at(starting).unwrap().height,
        }
    }
}

impl Iterator for GridIterator<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_pos) = self.grid.at(self.next) {
            let mut skip = false;

            match (&self.visibility, self.previous) {
                // if we're iterating by visibility then we need to also need to
                // short circuit if the previous land plot we saw was higher than
                // us, and if it's the same size we need to keep looking, but not
                // return the current value as a visible region.
                (VisibilityRules::PartOne, Some(_)) => {
                    match self.max_height.cmp(&next_pos.height) {
                        Ordering::Less => self.max_height = next_pos.height,
                        _ => skip = true,
                    }
                }

                // don't count ourselves, and short circuit the iterator in a
                // way that it returns the current, but can't continue once we
                // find something the same size, or taller than us.
                (VisibilityRules::PartTwo, previous) => match previous {
                    None => skip = true,
                    Some(_) => {
                        if self.max_height <= next_pos.height {
                            // modify the direction so that next lands us as
                            // (-1, -1), thereby short circuiting the next time
                            // next() is used.
                            self.direction = (-self.next.0 - 1, -self.next.1 - 1);
                        }
                    }
                },
                _ => (),
            }

            self.previous = Some(self.next);
            self.next = (
                self.next.0 + self.direction.0,
                self.next.1 + self.direction.1,
            );

            return match skip {
                true => self.next(),
                false => self.previous,
            };
        }

        None
    }
}

// roughly speaking, we'll iterator over the 4 edges and work inwards to
// determine what is visible, our iterator will stop when the next element is no
// longer visible.
pub fn part_one(input: &str) -> usize {
    let grid = Grid::from(input, LandPlot::new);
    let mut visibility: HashSet<(i32, i32)> = HashSet::new();

    let scans = [
        ((0, 0), EAST, SOUTH),
        ((0, 0), SOUTH, EAST),
        ((grid.max_width, grid.max_height), WEST, NORTH),
        ((grid.max_width, grid.max_height), NORTH, WEST),
    ];

    for (start_pos, dir, scan_dir) in scans {
        for edge_pos in GridIterator::new(&grid, start_pos, dir, VisibilityRules::None) {
            for scan_pos in GridIterator::new(&grid, edge_pos, scan_dir, VisibilityRules::PartOne) {
                visibility.insert(scan_pos);
            }
        }
    }

    visibility.len()
}

// we'll iterate through all the points. we'll then look in each direction to
// count the visible squares, multiply them, then figure out the max value.
pub fn part_two(input: &str) -> usize {
    let grid = Grid::from(input, LandPlot::new);

    grid.points
        .keys()
        .map(|coord| {
            count_visible(&grid, coord, NORTH)
                * count_visible(&grid, coord, SOUTH)
                * count_visible(&grid, coord, EAST)
                * count_visible(&grid, coord, WEST)
        })
        .max()
        .unwrap()
}

fn count_visible(grid: &Grid<LandPlot>, from: &(i32, i32), direction: (i32, i32)) -> usize {
    GridIterator::new(grid, *from, direction, VisibilityRules::PartTwo).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::fs;

    static EXAMPLE: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    fn read_input_file() -> String {
        fs::read_to_string("input.txt").expect("oops - file could not be read")
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(21, part_one(EXAMPLE));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1719, part_one(&read_input_file()));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(8, part_two(EXAMPLE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(590824, part_two(&read_input_file()));
    }
}
