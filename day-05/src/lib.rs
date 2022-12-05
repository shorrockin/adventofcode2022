use regex::Regex;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Stack {
    elements: Vec<char>,
}

pub fn part_one(input: &str, columns: usize) -> String {
    let (mut stacks, instructions) = parse(input, columns);
    for instruction in instructions {
        (0..instruction.amount).for_each(|_| {
            let removed = stacks[instruction.from].elements.remove(0);
            stacks[instruction.to].elements.insert(0, removed);
        });
    }

    stacks.iter().map(|stack| stack.elements[0]).collect()
}

pub fn part_two(input: &str, columns: usize) -> String {
    let (mut stacks, instructions) = parse(input, columns);
    for instruction in instructions {
        let removed: Vec<char> = stacks[instruction.from]
            .elements
            .drain(0..instruction.amount)
            .collect();

        removed
            .iter()
            .rev()
            .for_each(|r| stacks[instruction.to].elements.insert(0, *r));
    }

    stacks.iter().map(|stack| stack.elements[0]).collect()
}

fn parse(input: &str, columns: usize) -> (Vec<Stack>, Vec<Instruction>) {
    let (stack_layout, instructions_raw) = input.split_once("\n\n").unwrap();
    let mut stacks = vec![Stack { elements: vec![] }; columns];
    let instructions_regex = Regex::new(r"\d+").unwrap();

    for line in stack_layout.lines() {
        line.chars()
            .enumerate()
            .filter(|(_, character)| character.is_alphabetic())
            .for_each(|(index, character)| stacks[(index - 1) / 4].elements.push(character));
    }

    let instructions = instructions_raw
        .lines()
        .map(|line| {
            let numbers: Vec<usize> = instructions_regex
                .captures_iter(line)
                .map(|number| number[0].parse().unwrap())
                .collect();
            Instruction {
                amount: numbers[0],
                from: numbers[1] - 1, // offset by 1 for array indexing
                to: numbers[2] - 1,
            }
        })
        .collect();

    (stacks, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::fs;

    static EXAMPLE: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
        1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    fn read_input_file() -> String {
        fs::read_to_string("input.txt").expect("oops - file could not be read")
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!("CMZ", part_one(EXAMPLE, 3));
    }

    #[test]
    fn test_part_one() {
        assert_eq!("VGBBJCRMN", part_one(&read_input_file(), 9));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!("MCD", part_two(EXAMPLE, 3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!("LBBVJBRMH", part_two(&read_input_file(), 9));
    }
}
