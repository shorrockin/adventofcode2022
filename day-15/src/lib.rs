pub mod utils;
use crate::utils::grid::Coordinate;
use regex::Regex;

pub fn part_one(input: &str, row: i32) -> i32 {
    parse(input)
        .iter()
        .flat_map(|sensor| sensor.occupied_at_row(row))
        .fold(Range(i32::MAX, i32::MIN), |acc, range| acc.merge(&range))
        .size()
}

pub fn part_two(input: &str, max_size: i32) -> i64 {
    let sensors = parse(input);

    for row in 0..max_size {
        let ranges: Vec<_> = sensors
            .iter()
            .flat_map(|sensor| sensor.occupied_at_row(row))
            .collect();

        let potential_gap: Option<i32> = ranges
            .iter()
            .flat_map(|range| [range.0 - 1, range.1 + 1])
            .filter(|gap| gap > &0 && gap < &max_size)
            .find(|gap| !ranges.iter().any(|range| range.includes(*gap)));

        if let Some(x) = potential_gap {
            return ((x as i64) * 4_000_000) + (row as i64);
        }
    }

    unreachable!("never found the gap")
}

#[derive(Debug, Copy, Clone)]
struct Range(i32, i32);
impl Range {
    fn size(&self) -> i32 {
        self.1 - self.0
    }

    fn includes(&self, value: i32) -> bool {
        self.0 <= value && self.1 >= value
    }

    fn merge(&self, other: &Range) -> Range {
        Range(other.0.min(self.0), other.1.max(self.1))
    }
}

#[derive(Debug)]
struct Sensor {
    location: Coordinate,
    scan_distance: i32,
}

impl Sensor {
    fn new(location: Coordinate, beacon: Coordinate) -> Sensor {
        Sensor {
            location,
            scan_distance: location.distance(beacon),
        }
    }

    fn occupies_row(&self, row: i32) -> bool {
        let min = self.location.1 - self.scan_distance;
        let max = self.location.1 + self.scan_distance;
        row == row.min(max).max(min)
    }

    fn occupied_at_row(&self, row: i32) -> Option<Range> {
        if !self.occupies_row(row) {
            return None;
        }

        let distance_to_row = (self.location.1 - row).abs();
        Some(Range(
            self.location.0 - self.scan_distance + distance_to_row,
            self.location.0 + self.scan_distance - distance_to_row,
        ))
    }
}

fn parse(input: &str) -> Vec<Sensor> {
    let re = Regex::new(r"(-?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = re
                .captures_iter(line)
                .map(|c| c[1].parse().unwrap())
                .collect();
            Sensor::new(
                Coordinate(numbers[0], numbers[1]),
                Coordinate(numbers[2], numbers[3]),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(26, part_one(EXAMPLE_INPUT, 10));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(4919281, part_one(INPUT, 2_000_000));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(56000011, part_two(EXAMPLE_INPUT, 20));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(12630143363767, part_two(INPUT, 4_000_000));
    }
}
