#![allow(dead_code, unused_variables, unused_imports)]
use itertools::Itertools;
use pathfinding::prelude::bfs;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

static START_VALVE: &str = "AA";

#[derive(Debug)]
struct Valve {
    name: String,
    flow: usize,
    connections: Vec<String>,
    distances: HashMap<String, usize>,
}
impl Valve {
    fn record_distance(&mut self, to: String, cost: usize) {
        self.distances.insert(to, cost);
    }

    fn distance(&self, to: &str) -> usize {
        match &self.distances.get(to) {
            Some(value) => **value,
            None => panic!(
                "unable to get distances for {} in {}, distances didn't contain it: {:?}",
                to, self.name, self.distances
            ),
        }
    }
}

#[derive(Debug)]
struct PathResult {
    time_taken: usize,
    pressure: usize,
    path: Vec<String>,
}
impl PathResult {
    fn new() -> PathResult {
        PathResult {
            time_taken: 0,
            pressure: 0,
            path: vec!["AA".to_string()],
        }
    }

    fn cmp(&self, other: &Option<PathResult>) -> Ordering {
        match other {
            Some(pr) => match (
                self.pressure.cmp(&pr.pressure),
                self.time_taken.cmp(&pr.time_taken),
            ) {
                (Ordering::Greater, _) => Ordering::Greater,
                (Ordering::Less, _) => Ordering::Less,
                (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
                (Ordering::Equal, Ordering::Less) => Ordering::Less,
                (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            },
            None => Ordering::Greater,
        }
    }
}

#[derive(Debug)]
struct Valves(HashMap<String, Valve>);
impl Valves {
    fn at(&self, location: &str) -> &Valve {
        self.0.get(location).unwrap()
    }

    fn at_mut(&mut self, location: &str) -> &mut Valve {
        self.0.get_mut(location).unwrap()
    }

    fn pressure_valves(&self) -> Vec<&Valve> {
        self.0.values().filter(|v| v.flow > 0).collect()
    }
}

pub fn part_one(input: &str) -> usize {
    let valves = parse(input);
    let pressure_valves = valves.pressure_valves();

    let start = valves.at(START_VALVE);
    match best_moves(start, &valves, 0, 0, vec![]) {
        Some(result) => {
            dbg!(&result);
            result.pressure
        }
        None => panic!("got None from best_moves"),
    }
}

pub fn part_two(_input: &str) -> usize {
    0
}

fn best_moves(
    current: &Valve,
    valves: &Valves,
    time_taken: usize,
    pressure: usize,
    path: Vec<String>,
) -> Option<PathResult> {
    let mut best: Option<PathResult> = None;
    // println!(
    //     "{}: from {:?}, time_taken: {}, pressure: {}",
    //     current.name, path, time_taken, pressure
    // );

    for child_str in current.distances.keys() {
        let child = valves.at(child_str);

        // don't consider moving to a non-flow child valve
        if child.flow == 0 {
            continue;
        }

        // if we're going to exceed our max time, then we can't travel to this child
        let distance = current.distance(child_str);
        if time_taken + distance + 1 > 30 {
            continue;
        }

        // if we've already traveled to this child previously, it doesn't make
        // sense to revisit as it's already on
        if path.contains(child_str) {
            continue;
        }

        let mut child_path = path.clone();
        child_path.push(child_str.to_string());

        let child_time_taken = time_taken + distance + 1; // +1 to turn on
        let child_pressure = pressure + ((30 - child_time_taken) * child.flow);
        let child_best_move =
            best_moves(child, valves, child_time_taken, child_pressure, child_path);

        if current.name.eq("AA") {
            dbg!(&child_best_move);
        }

        best = match child_best_move {
            None => best,
            Some(bm) => match bm.cmp(&best) {
                Ordering::Greater => Some(bm),
                Ordering::Less => best,
                Ordering::Equal => best,
            },
        }
    }

    match best {
        None => Some(PathResult {
            time_taken,
            pressure,
            path,
        }),
        Some(best) => Some(best),
    }
}

// parses our input structure into a hashmap where the name is the name of the
// valve for easier lookup.s
fn parse(input: &str) -> Valves {
    let pattern = r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)";
    let regex = Regex::new(pattern).unwrap();
    let mut valves = Valves(
        input
            .lines()
            .flat_map(|line| {
                regex.captures_iter(line).map(|cap| Valve {
                    name: cap[1].to_string(),
                    flow: cap[2].parse().unwrap(),
                    connections: cap[3].split(',').map(|v| v.trim().to_string()).collect(),
                    distances: HashMap::new(),
                })
            })
            .map(|valve| (valve.name.to_string(), valve))
            .collect(),
    );

    combinations(valves.pressure_valves())
        .iter()
        .for_each(|(from, to)| {
            let distance = measure_distance(&valves, from, to);
            let from_valve = valves.at_mut(from);
            from_valve.record_distance(to.to_string(), distance);

            let to_valve = valves.at_mut(to);
            to_valve.record_distance(from.to_string(), distance);
        });

    valves
}

// calculates the movement distance between two coordinates
fn measure_distance(valves: &Valves, from: &String, to: &String) -> usize {
    let path = bfs(
        from,
        |source| valves.at(source).connections.clone(),
        |node| node == to,
    );

    match path {
        Some(value) => value.len() - 1,
        None => panic!("expected path {} -> {} but got None", from, to),
    }
}

// returns all the possible combinations between strings provided
fn combinations(valves: Vec<&Valve>) -> Vec<(String, String)> {
    let mut combinations = Vec::new();
    for i in 0..valves.len() {
        combinations.push((START_VALVE.to_string(), valves[i].name.clone()));
        for j in i + 1..valves.len() {
            combinations.push((valves[i].name.clone(), valves[j].name.clone()));
        }
    }
    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        let valves = parse(EXAMPLE_INPUT);
        assert_eq!(5, valves.at("AA").distance("HH"));
        assert_eq!(1, valves.at("AA").distance("DD"));
        assert_eq!(2, valves.at("AA").distance("CC"));
        assert_eq!(10, valves.0.len());

        assert_eq!(1650, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        // assert_eq!(2080, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        // assert_eq!(1707, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(99, part_two(INPUT));
    }
}
