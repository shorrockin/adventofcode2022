use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Coordinate(i32, i32, i32); // x, y, z
impl Coordinate {
    fn from_input(input: &str) -> HashSet<Coordinate> {
        input
            .lines()
            .map(|line| line.split(','))
            .map(|mut split| {
                Coordinate(
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    // returns a set of "neighbor" coordinates which are not included in the
    // provided coordinates set.
    fn empty_neighbors(&self, coordinates: &HashSet<Coordinate>) -> HashSet<Coordinate> {
        [
            (1, 0, 0),  // right
            (-1, 0, 0), // left
            (0, 1, 0),  // south
            (0, -1, 0), // north
            (0, 0, 1),  // towards
            (0, 0, -1), // away
        ]
        .map(|(x, y, z)| Coordinate(self.0 + x, self.1 + y, self.2 + z))
        .into_iter()
        .filter(|coordinate| !coordinates.contains(coordinate))
        .collect()
    }
}

// parse all coordinates, retrieve all empty neighbors, then sum them up. simple.
pub fn part_one(input: &str) -> i32 {
    let coordinates = Coordinate::from_input(input);
    coordinates
        .iter()
        .map(|coordinate| coordinate.empty_neighbors(&coordinates).len() as i32)
        .sum()
}

// parse all coordinates. find an outside corner coordinate that we know we can
// fill from. start to fill empty spaces not exceeding the outside bounds of the
// coordinate system. avoid recursion as the stack will get massive, use a queue
// system instead. once we've filled in, we can count empty neighbors like part
// 1 and cross check against the flood fill.
pub fn part_two(input: &str) -> i32 {
    let coordinates = Coordinate::from_input(input);
    let max_x = max_coordinate(&coordinates, |l, r| l.0.cmp(&r.0)).0 + 1;
    let min_x = max_coordinate(&coordinates, |l, r| r.0.cmp(&l.0)).0 - 1;
    let max_y = max_coordinate(&coordinates, |l, r| l.1.cmp(&r.1)).1 + 1;
    let min_y = max_coordinate(&coordinates, |l, r| r.1.cmp(&l.1)).1 - 1;
    let max_z = max_coordinate(&coordinates, |l, r| l.2.cmp(&r.2)).2 + 1;
    let min_z = max_coordinate(&coordinates, |l, r| r.2.cmp(&l.2)).2 - 1;

    let mut to_flood: Vec<Coordinate> = Vec::new(); // all the coordinates we need to process
    let mut flooded: HashSet<Coordinate> = HashSet::new(); // everything we've already marked as flooded
    to_flood.push(Coordinate(max_x, max_y, max_z));

    while let Some(current) = to_flood.pop() {
        current
            .empty_neighbors(&coordinates)
            .iter()
            .filter(|coordinate| {
                !flooded.contains(coordinate)
                    && coordinate.0 <= max_x
                    && coordinate.0 >= min_x
                    && coordinate.1 <= max_y
                    && coordinate.1 >= min_y
                    && coordinate.2 <= max_z
                    && coordinate.2 >= min_z
            })
            .for_each(|coord| to_flood.push(*coord));

        flooded.insert(current);
    }

    coordinates
        .iter()
        .map(|coordinate| {
            coordinate
                .empty_neighbors(&coordinates)
                .iter()
                .filter(|coordinate| flooded.contains(coordinate))
                .count() as i32
        })
        .sum()
}

fn max_coordinate<F>(coordinates: &HashSet<Coordinate>, compare: F) -> Coordinate
where
    F: Fn(&Coordinate, &Coordinate) -> Ordering,
{
    *coordinates
        .iter()
        .max_by(|left, right| compare(left, right))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(64, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(4504, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(58, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2556, part_two(INPUT));
    }
}
