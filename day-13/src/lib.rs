use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Less};
use Component::{List, Value};

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn new(left: &str, right: &str) -> PacketPair {
        PacketPair {
            left: Packet::new(left),
            right: Packet::new(right),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    components: Vec<Component>,
}

impl Packet {
    fn new(input: &str) -> Packet {
        // converts the input to a sequence of symbols which are either square
        // brackets or a number.
        let binding = input.replace('[', ",[,").replace(']', ",],");
        let symbols = binding
            .split(',')
            .filter(|str| !str.is_empty())
            .collect::<Vec<_>>();

        let mut previous: Vec<Vec<Component>> = vec![];
        let mut current = vec![];

        for symbol in symbols {
            match symbol {
                "[" => {
                    let next: Vec<Component> = vec![];
                    previous.push(current);
                    current = next;
                }
                "]" => {
                    let mut parent = previous.pop().unwrap();
                    parent.push(List(current));
                    current = parent;
                }
                _ => {
                    current.push(Value(symbol.parse::<u32>().unwrap()));
                }
            }
        }
        let current_list = match current.get(0) {
            Some(List(values)) => values,
            _ => panic!("current should be a list after parsing"),
        };

        Packet {
            components: current_list.to_vec(),
        }
    }

    fn cmp(&self, other: &Packet) -> Ordering {
        List(self.components.to_vec()).cmp(&List(other.components.to_vec()))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Component {
    List(Vec<Component>),
    Value(u32),
}

impl Component {
    fn cmp(&self, other: &Component) -> Ordering {
        match (self, other) {
            (Value(left), Value(right)) => left.cmp(right),
            (List(left_vec), List(right_vec)) => {
                let mut result = Equal;
                for (left, right) in left_vec.iter().zip(right_vec.iter()) {
                    result = left.cmp(right);
                    if result != Equal {
                        break;
                    }
                }

                // zip only returns a tuple of matched elements, so for Vec of
                // different sizes, after we check, if any are still unordered
                // delegate to a length check.
                if result == Equal {
                    result = left_vec.len().cmp(&right_vec.len())
                }

                result
            }
            (List(_), Value(right_val)) => self.cmp(&List(vec![Value(*right_val)])),
            (Value(left_val), List(_)) => List(vec![Value(*left_val)]).cmp(other),
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let packet_pairs: Vec<_> = input
        .split("\n\n")
        .map(|packet_pair| packet_pair.split_once('\n').unwrap())
        .map(|(left, right)| PacketPair::new(left, right))
        .collect();

    packet_pairs
        .iter()
        .enumerate()
        .map(|(idx, packet_pair)| {
            let left_vec = &packet_pair.left.components;
            let right_vec = &packet_pair.right.components;
            match List(left_vec.to_vec()).cmp(&List(right_vec.to_vec())) {
                Less => idx + 1,
                _ => 0,
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut input_with_dividers = "[[2]]\n[[6]]\n".to_string();
    input_with_dividers.push_str(input);

    let packets: Vec<_> = input_with_dividers
        .split('\n')
        .filter(|str| !str.is_empty())
        .map(Packet::new)
        .sorted_by(|left, right| left.cmp(right))
        .collect();

    let divider_one = packets.iter().position(|p| p.eq(&Packet::new("[[2]]")));
    let divider_two = packets.iter().position(|p| p.eq(&Packet::new("[[6]]")));

    (divider_one.unwrap() + 1) * (divider_two.unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(13, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(5390, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(140, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(19261, part_two(INPUT));
    }
}
