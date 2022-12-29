#[derive(Debug)]
struct Entry {
    value: i64,
    order: usize,
}
impl Entry {
    fn new(value: &str, order: usize, decryption_key: i64) -> Entry {
        Entry {
            value: value.parse::<i64>().unwrap() * decryption_key,
            order,
        }
    }
}

#[derive(Debug)]
struct Entries(Vec<Entry>);
impl Entries {
    fn new(input: &str, decryption_key: i64) -> Entries {
        Entries(
            input
                .lines()
                .enumerate()
                .map(|(order, line)| Entry::new(line, order, decryption_key))
                .collect(),
        )
    }

    fn at_order_index(&self, idx: usize) -> Option<(&Entry, usize)> {
        self.0
            .iter()
            .position(|e| e.order == idx)
            .map(|e| (self.0.get(e).unwrap(), e))
    }

    fn mix(&mut self, times: usize) {
        for _ in 0..times {
            let mut current_order_idx = 0;

            while let Some((current, move_from)) = self.at_order_index(current_order_idx) {
                let relative_move = move_from as i64 + current.value;
                let modified_len = self.0.len() as i64 - 1; // offset by 1 for future removal
                let move_to = (relative_move.rem_euclid(modified_len)) as usize;

                let removed = self.0.remove(move_from);
                self.0.insert(move_to, removed);

                current_order_idx += 1;
            }
        }
    }

    fn value_at(&self, idx: usize) -> i64 {
        self.0.get(idx % self.0.len()).unwrap().value
    }

    fn grove_coordinates(&self) -> i64 {
        let zero_position = self.0.iter().position(|entry| entry.value == 0).unwrap();
        self.value_at(zero_position + 1000)
            + self.value_at(zero_position + 2000)
            + self.value_at(zero_position + 3000)
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut entries = Entries::new(input, 1);
    entries.mix(1);
    entries.grove_coordinates()
}

pub fn part_two(input: &str) -> i64 {
    let mut entries = Entries::new(input, 811589153);
    entries.mix(10);
    entries.grove_coordinates()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(3, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(13883, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(1623178306, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(19185967576920, part_two(INPUT));
    }
}
