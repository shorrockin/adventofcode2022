#[derive(Debug)]
enum Command {
    Noop,
    Add(i32),
}

type Commands = Vec<Command>;

fn parse(input: &str) -> Commands {
    input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .flat_map(|line: Vec<_>| match line[0] {
            "noop" => vec![Command::Noop],
            "addx" => vec![Command::Noop, Command::Add(line[1].parse::<i32>().unwrap())],
            _ => panic!("unrecognized command: {}", line[0]),
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    static BREAKPOINTS: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let commands = parse(input);
    let mut signal_strength: i32 = 0;
    let mut register: i32 = 1;

    for (cycle, command) in commands.iter().enumerate() {
        if BREAKPOINTS.contains(&(cycle + 1)) {
            signal_strength += ((cycle + 1) as i32) * register;
        }

        match command {
            Command::Noop => (),
            Command::Add(amount) => register += amount,
        }
    }

    signal_strength
}

pub fn part_two(input: &str) {
    let commands = parse(input);
    let mut register: i32 = 1;
    let mut result = ['X'; 240];

    for (cycle, command) in commands.iter().enumerate() {
        let horizontal_pos = (cycle as i32) % 40;
        match (horizontal_pos == register)
            || (horizontal_pos == register + 1)
            || (horizontal_pos == register - 1)
        {
            true => result[cycle] = '#',
            false => result[cycle] = '.',
        }

        match command {
            Command::Noop => (),
            Command::Add(amount) => register += amount,
        }
    }

    for chunk in result.chunks(40) {
        println!("{}", chunk.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../example.input.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(13140, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(14780, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        part_two(EXAMPLE_INPUT);
    }

    #[test]
    fn test_part_two() {
        part_two(INPUT);
    }
}
