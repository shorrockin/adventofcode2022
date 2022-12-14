use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    solve(input, vec![(0, 0); 2])
}

pub fn part_two(input: &str) -> usize {
    solve(input, vec![(0, 0); 10])
}

fn solve(input: &str, mut rope: Rope) -> usize {
    let mut visited: HashSet<Coordinate> = HashSet::from_iter(vec![(0, 0)]);

    input
        .lines()
        .map(|line| {
            let command: Vec<_> = line.split_whitespace().collect();
            (command[0], command[1].parse::<i32>().unwrap())
        })
        .for_each(|split_line| match split_line {
            ("R", num_moves) => move_head(&mut rope, (1, 0), num_moves, &mut visited),
            ("U", num_moves) => move_head(&mut rope, (0, -1), num_moves, &mut visited),
            ("D", num_moves) => move_head(&mut rope, (0, 1), num_moves, &mut visited),
            ("L", num_moves) => move_head(&mut rope, (-1, 0), num_moves, &mut visited),
            _ => panic!("unknown direction {:?}", split_line),
        });

    visited.len()
}

fn move_head(
    rope: &mut Rope,
    direction: (i32, i32),
    num_moves: i32,
    visited: &mut HashSet<Coordinate>,
) {
    for _ in 0..num_moves {
        for idx in 0..rope.len() {
            match idx {
                0 => rope[0] = rope[0].move_by(&direction),
                _ => {
                    let previous = rope[idx - 1];
                    let current = rope[idx];
                    let delta = previous.delta(&current);
                    if delta > 1 {
                        let move_x = (previous.0 - current.0).signum();
                        let move_y = (previous.1 - current.1).signum();
                        rope[idx] = current.move_by(&(move_x, move_y));

                        // track if the tail moves only
                        if idx == (rope.len() - 1) {
                            visited.insert(rope[idx]);
                        }
                    }
                }
            }
        }
    }
}

type Coordinate = (i32, i32);
type Rope = Vec<Coordinate>;

// something that is plottable in a space
trait Plottable {
    fn delta(&self, other: &Self) -> i32;
    fn move_by(&self, distance: &Self) -> Self;
}

impl Plottable for (i32, i32) {
    fn delta(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs().max((self.1 - other.1).abs())
    }

    fn move_by(&self, distance: &Self) -> Self {
        ((self.0 + distance.0), (self.1 + distance.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_PART_ONE: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    static EXAMPLE_PART_TWO: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    fn read_input_file() -> String {
        include_str!("../input.txt").to_string()
    }

    #[test]
    fn test_utils() {
        assert_eq!(1, (0, 0).delta(&(1, 1)));
        assert_eq!(1, (0, 0).delta(&(0, 1)));
        assert_eq!(2, (0, 0).delta(&(-2, -1)));
        assert_eq!((3, 2), (1, 1).move_by(&(2, 1)));
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(13, part_one(EXAMPLE_PART_ONE));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(5902, part_one(&read_input_file()));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(1, part_two(EXAMPLE_PART_ONE));
        assert_eq!(36, part_two(EXAMPLE_PART_TWO));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2445, part_two(&read_input_file()));
    }
}
