use core::fmt;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};

// our core model for the location in grid, effectively a tuple with utility.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}
impl Location {
    fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location({}, {})", self.x, self.y)
    }
}

// drives behaviour of the solution
enum Part {
    One,
    Two,
}

// enum used to encode the move decision, will either have an elf moving to a
// square, doing nothing because they are done and nothing is around them, or they are
// conflicted and can't move because another elf is already moving to this square.
enum Proposal {
    Move(Location, Location), // from, to
    Stay,
    Done,
}

// encodes our various directional options, apply method used to do the
// coordinate math and return the new hypothetical elf.
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl Direction {
    fn apply(&self, loc: &Location) -> Location {
        match self {
            Direction::North => Location::new(loc.x, loc.y - 1),
            Direction::NorthEast => Location::new(loc.x + 1, loc.y - 1),
            Direction::East => Location::new(loc.x + 1, loc.y),
            Direction::SouthEast => Location::new(loc.x + 1, loc.y + 1),
            Direction::South => Location::new(loc.x, loc.y + 1),
            Direction::SouthWest => Location::new(loc.x - 1, loc.y + 1),
            Direction::West => Location::new(loc.x - 1, loc.y),
            Direction::NorthWest => Location::new(loc.x - 1, loc.y - 1),
        }
    }
}

fn parse(input: &str) -> HashSet<Location> {
    let mut elves = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, symbol)| {
            if symbol == '#' {
                elves.insert(Location::new(x as i32, y as i32));
            }
        })
    });

    elves
}

fn propose(elves: &HashSet<Location>, elf: &Location, turn: usize) -> Proposal {
    let free = [
        !elves.contains(&Direction::North.apply(elf)), // N: 0
        !elves.contains(&Direction::NorthEast.apply(elf)), // NE: 1
        !elves.contains(&Direction::East.apply(elf)),  // E: 2
        !elves.contains(&Direction::SouthEast.apply(elf)), // SE: 3
        !elves.contains(&Direction::South.apply(elf)), // S: 4
        !elves.contains(&Direction::SouthWest.apply(elf)), // SW: 5
        !elves.contains(&Direction::West.apply(elf)),  // W: 6
        !elves.contains(&Direction::NorthWest.apply(elf)), // NW: 7
    ];

    // nobody around us, we can just not move
    if !free.contains(&false) {
        return Proposal::Done;
    }

    for cycle in 0..4 {
        let adjusted_cycle = (turn + cycle) % 4;
        match adjusted_cycle {
            0 if free[0] && free[1] && free[7] => {
                return Proposal::Move(*elf, Direction::North.apply(elf));
            }
            1 if free[3] && free[4] && free[5] => {
                return Proposal::Move(*elf, Direction::South.apply(elf));
            }
            2 if free[5] && free[6] && free[7] => {
                return Proposal::Move(*elf, Direction::West.apply(elf));
            }
            3 if free[1] && free[2] && free[3] => {
                return Proposal::Move(*elf, Direction::East.apply(elf));
            }
            _ => (),
        };
    }

    Proposal::Stay
}

fn solve(input: &str, part: Part) -> i32 {
    let mut elves = parse(input);
    let mut moves: HashMap<Location, Location> = HashMap::with_capacity(elves.len());
    let mut conflicts: HashSet<Location> = HashSet::new();
    let mut done_count: usize;
    let mut turn = 0;

    loop {
        done_count = 0;
        moves.clear();
        conflicts.clear();

        // first calculate all our proposals into both the moves and the
        // conflicts collection
        for elf in &elves {
            match propose(&elves, elf, turn) {
                Proposal::Move(from, to) if !conflicts.contains(&to) => {
                    if let Vacant(entry) = moves.entry(to) {
                        entry.insert(from);
                    } else {
                        moves.remove(&to);
                        conflicts.insert(to);
                    }
                }
                Proposal::Done => done_count += 1,
                _ => (),
            }
        }

        // execute the moves, ignore any conflicts
        for elf_move in &moves {
            assert!(elves.remove(elf_move.1), "could not remove elf at location");
            elves.insert(*elf_move.0);
        }

        turn += 1;

        match part {
            Part::One if turn == 10 => {
                let elves_vec: Vec<_> = elves.into_iter().collect();
                let min_x = elves_vec.iter().map(|location| location.x).min().unwrap();
                let max_x = elves_vec.iter().map(|location| location.x).max().unwrap();
                let min_y = elves_vec.iter().map(|location| location.y).min().unwrap();
                let max_y = elves_vec.iter().map(|location| location.y).max().unwrap();

                return ((max_x - min_x + 1) * (max_y - min_y + 1)) - elves_vec.len() as i32;
            }
            Part::Two if done_count == elves.len() => {
                return turn as i32;
            }
            _ => {}
        }
    }
}

pub fn part_one(input: &str) -> i32 {
    solve(input, Part::One)
}

pub fn part_two(input: &str) -> i32 {
    solve(input, Part::Two)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT_ONE: &str = include_str!("../input.example.one.txt");
    static EXAMPLE_INPUT_TWO: &str = include_str!("../input.example.two.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(25, part_one(EXAMPLE_INPUT_ONE));
        assert_eq!(110, part_one(EXAMPLE_INPUT_TWO));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(4005, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(20, part_two(EXAMPLE_INPUT_TWO));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1008, part_two(INPUT));
    }
}
