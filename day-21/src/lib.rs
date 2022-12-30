use num::{Complex, Zero};
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Number(f64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}
impl Instruction {
    fn eval(&self, context: &Monkeys) -> Complex<f64> {
        match self {
            Instruction::Number(value) => Complex::new(*value, 0.0),
            Instruction::Add(left, right) => context.eval(left) + context.eval(right),
            Instruction::Subtract(left, right) => context.eval(left) - context.eval(right),
            Instruction::Multiply(left, right) => {
                let eval_right = context.eval(right);
                let eval_left = context.eval(left);
                match (eval_left.im.is_zero(), eval_right.im.is_zero()) {
                    (true, false) => eval_right.scale(eval_left.re),
                    (_, true) => eval_left.scale(eval_right.re),
                    _ => panic!("should not have cases where both sides contain imaginary numbers"),
                }
            }
            Instruction::Divide(left, right) => {
                let eval_left = context.eval(left);
                let eval_right = context.eval(right);
                match (eval_left.im.is_zero(), eval_right.im.is_zero()) {
                    (_, true) => eval_left.unscale(eval_right.re),
                    _ => panic!("we don't have cases where the imaginary is on the left"),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    instruction: Instruction,
    human: bool,
}
impl Monkey {
    fn new(human: bool, instruction: Instruction) -> Monkey {
        Monkey { instruction, human }
    }

    fn eval(&self, context: &Monkeys) -> Complex<f64> {
        match self.human {
            true => Complex::i(),
            false => self.instruction.eval(context),
        }
    }
}

#[derive(Debug)]
struct Monkeys(HashMap<String, Monkey>);
impl Monkeys {
    fn new(input: &str, parse_human: bool) -> Monkeys {
        Monkeys(
            input
                .lines()
                .map(|line| {
                    let (name, operation_str) = line.split_once(": ").unwrap();
                    let operation = match operation_str.split(' ').collect::<Vec<_>>().as_slice() {
                        [value] => Instruction::Number(value.parse().unwrap()),
                        [left, "+", right] => Instruction::Add(left.to_string(), right.to_string()),
                        [left, "-", right] => {
                            Instruction::Subtract(left.to_string(), right.to_string())
                        }
                        [left, "*", right] => {
                            Instruction::Multiply(left.to_string(), right.to_string())
                        }
                        [left, "/", right] => {
                            Instruction::Divide(left.to_string(), right.to_string())
                        }
                        _ => panic!(),
                    };
                    let monkey = Monkey::new(parse_human && name.eq("humn"), operation);
                    (name.to_string(), monkey)
                })
                .collect(),
        )
    }

    fn eval(&self, name: &str) -> Complex<f64> {
        let monkey = self.0.get(name).unwrap();
        monkey.eval(self)
    }
}

pub fn part_one(input: &str) -> f64 {
    let monkeys = Monkeys::new(input, false);
    monkeys.eval("root").re
}

pub fn part_two(input: &str) -> f64 {
    let monkeys = Monkeys::new(input, true);
    let root = monkeys.0.get("root").unwrap();
    match &root.instruction {
        Instruction::Add(left, right) => {
            let left_monkey = monkeys.0.get(left).unwrap();
            let right_monkey = monkeys.0.get(right).unwrap();
            let left_eval = left_monkey.eval(&monkeys);
            let right_eval = right_monkey.eval(&monkeys);

            match (left_eval.im.is_zero(), right_eval.im.is_zero()) {
                (true, false) => ((left_eval.re - right_eval.re) / right_eval.im).round(),
                (false, true) => ((right_eval.re - left_eval.re) / left_eval.im).round(),
                _ => panic!(
                    "expected one imaginary number in either the left or right complex number"
                ),
            }
        }
        _ => panic!("root didn't have an add operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(152.0, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(282285213953670.0, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(301.0, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3699945358564.0, part_two(INPUT));
    }
}
