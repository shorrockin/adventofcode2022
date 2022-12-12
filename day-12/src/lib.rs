pub mod utils;
use crate::utils::grid::{Coordinate, Grid, Point};
use pathfinding::prelude::bfs;

pub fn part_one(input: &str) -> usize {
    let grid = Grid::from(input, ElevatedPoint::new);
    let start = grid
        .points
        .iter()
        .find(|point| point.1.path_type == PathType::Start)
        .unwrap()
        .1;

    solve(&grid, start).unwrap().len() - 1
}

pub fn part_two(input: &str) -> usize {
    let grid = Grid::from(input, ElevatedPoint::new);
    let starts: Vec<_> = grid
        .points
        .iter()
        .map(|point| point.1)
        .filter(|point| point.height == 10)
        .collect();

    starts
        .iter()
        .map(|start| solve(&grid, start))
        .filter(|path| path.is_some())
        .map(|path| path.unwrap().len() - 1)
        .min()
        .unwrap()
}

fn solve(grid: &Grid<ElevatedPoint>, start: &ElevatedPoint) -> Option<Vec<ElevatedPoint>> {
    let successors = |point: &ElevatedPoint| -> Vec<ElevatedPoint> {
        vec![
            grid.north(point),
            grid.south(point),
            grid.east(point),
            grid.west(point),
        ]
        .iter()
        .filter_map(|n| *n)
        .filter(|neighbor| (neighbor.height - point.height) <= 1)
        .copied()
        .collect()
    };

    bfs(start, successors, ElevatedPoint::is_end)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PathType {
    Start,
    Path,
    End,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ElevatedPoint {
    coord: Coordinate,
    path_type: PathType,
    height: i32,
}

impl ElevatedPoint {
    fn new(coord: Coordinate, height: char) -> ElevatedPoint {
        let (path_type, height) = match height {
            'S' => (PathType::Start, 10), // 10 == value of 'a'
            'E' => (PathType::End, 35),   // 35 == value of 'z'
            _ => (PathType::Path, height.to_digit(36).unwrap() as i32),
        };

        ElevatedPoint {
            coord,
            path_type,
            height,
        }
    }

    fn is_end(&self) -> bool {
        self.path_type == PathType::End
    }
}

impl Point for ElevatedPoint {
    fn symbol(&self) -> String {
        char::from_digit(self.height as u32, 36)
            .unwrap()
            .to_string()
    }

    fn coord(&self) -> Coordinate {
        self.coord
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(31, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(352, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(29, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(345, part_two(INPUT));
    }
}
