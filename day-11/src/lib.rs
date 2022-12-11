#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    throw_true: usize,
    throw_false: usize,
    inspections: u64,
}

#[derive(Debug)]
enum Operation {
    Square,
    Multiply(u64),
    Sum(u64),
}

pub fn part_one(input: &str) -> u64 {
    solve(input, 20, 3)
}

pub fn part_two(input: &str) -> u64 {
    solve(input, 10000, 1)
}

fn solve(input: &str, rounds: usize, div_amount: u64) -> u64 {
    let mut monkeys = parse(input);
    let lcm = monkeys.iter().map(|m| m.test).product::<u64>();

    for _round in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            let monkey = monkeys.get_mut(monkey_idx).unwrap();

            let passes: Vec<_> = monkey
                .items
                .iter()
                .map(|item| {
                    let worry_level = match monkey.operation {
                        Operation::Square => item.wrapping_mul(*item) / div_amount,
                        Operation::Multiply(amount) => item.wrapping_mul(amount) / div_amount,
                        Operation::Sum(amount) => item.wrapping_add(amount) / div_amount,
                    };
                    match (worry_level % monkey.test) == 0 {
                        true => (monkey.throw_true, worry_level % lcm),
                        false => (monkey.throw_false, worry_level % lcm),
                    }
                })
                .collect();

            monkey.inspections += monkey.items.len() as u64;
            monkey.items.clear();
            for (target, item) in passes {
                monkeys[target].items.push(item)
            }
        }
    }

    monkeys.sort_by(|left, right| right.inspections.cmp(&left.inspections));
    monkeys.iter().take(2).map(|m| m.inspections).product()
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_str| {
            let lines: Vec<_> = monkey_str.split("\n").collect();
            let items = lines[1]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect();
            let operation = lines[2]
                .split_once("old ")
                .unwrap()
                .1
                .split_once(' ')
                .map(|(operator, amount)| match (operator, amount) {
                    ("*", "old") => Operation::Square,
                    ("*", _) => Operation::Multiply(amount.parse::<u64>().unwrap()),
                    ("+", _) => Operation::Sum(amount.parse::<u64>().unwrap()),
                    _ => panic!("unknown operator: {}", operator),
                })
                .unwrap();
            let test = lines[3]
                .split_once("divisible by ")
                .map(|(_, amount)| amount.parse::<u64>().unwrap())
                .unwrap();
            let throw_true = lines[4]
                .split_once("throw to monkey ")
                .map(|(_, amount)| amount.parse::<usize>().unwrap())
                .unwrap();
            let throw_false = lines[5]
                .split_once("throw to monkey ")
                .map(|(_, amount)| amount.parse::<usize>().unwrap())
                .unwrap();

            Monkey {
                items,
                operation,
                test,
                throw_true,
                throw_false,
                inspections: 0,
            }
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
        assert_eq!(10605, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(121450, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(2713310158, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(28244037010, part_two(INPUT));
    }
}
