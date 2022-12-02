use std::fs;

fn main() {
    println!("Part 1 Result: {:?}", part1(&read_input_file()));
    println!("Part 2 Result: {:?}", part2(&read_input_file()));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn new(symbol: &str) -> Option<Choice> {
        match symbol {
            "A" => Some(Choice::Rock),
            "X" => Some(Choice::Rock),
            "B" => Some(Choice::Paper),
            "Y" => Some(Choice::Paper),
            "C" => Some(Choice::Scissors),
            "Z" => Some(Choice::Scissors),
            _ => None
        }
    }
    
    fn points(&self) -> usize {
        match &self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3
        }
    }

    fn wins_to(&self) -> Choice {
        match &self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper
        }
    }

    fn loses_to(&self) -> Choice {
        match &self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock
        }
    }
}

#[derive(Debug)]
enum Result {
    Won,
    Loss,
    Tie
}

impl Result {
    fn new(symbol: &str) -> Option<Result> {
        match symbol {
            "X" => Some(Result::Loss),
            "Y" => Some(Result::Tie), 
            "Z" => Some(Result::Won),
            _ => None
        }
    }
    
    fn points(&self) -> usize {
        match &self {
            Result::Won => 6,
            Result::Loss => 0,
            Result::Tie => 3
        }
    }
}

#[derive(Debug)]
struct Game {
    left: Choice,
    right: Choice
}

impl Game {
    fn part_1(line: &str) -> Game {
        let characters: Vec<&str> = line.split(' ').collect();
        Game { 
            left: Choice::new(characters[0]).unwrap(), 
            right: Choice::new(characters[1]).unwrap()
        }
    }

    fn part_2(line: &str) -> Game {
        let characters: Vec<&str> = line.split(' ').collect();
        let expectation = Result::new(characters[1]).unwrap();
        let left = Choice::new(characters[0]).unwrap();

        let right = match expectation {
            Result::Won => left.loses_to(),
            Result::Loss => left.wins_to(),
            Result::Tie => left
        };

        Game { left, right } 
    }

    fn play(&self) -> usize {
        let result = if self.right.wins_to() == self.left {
            Result::Won
        } else if self.right.loses_to() == self.left {
            Result::Loss
        } else {
            Result::Tie
        };
        result.points() + self.right.points()
    }
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("oops - file could not be read")
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::part_1(line).play())
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::part_2(line).play())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn test_part_1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_1() {
        let result = part1(&read_input_file());
        assert_eq!(result, 14827);
    }

    #[test]
    fn test_part_2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_2() {
        let result = part2(&read_input_file());
        assert_eq!(result, 13889);
    }
}
