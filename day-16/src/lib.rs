use pathfinding::prelude::bfs;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

static START_VALVE: &str = "AA";

#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, Eq, PartialEq)]
struct PathResult {
    time_taken: usize,
    pressure: usize,
    path: Vec<String>,
}
impl Ord for PathResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match (
            self.pressure.cmp(&other.pressure),
            self.time_taken.cmp(&other.time_taken),
        ) {
            (Ordering::Greater, _) => Ordering::Greater,
            (Ordering::Less, _) => Ordering::Less,
            (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
            (Ordering::Equal, Ordering::Less) => Ordering::Less,
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
        }
    }
}
impl PartialOrd for PathResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    let start = valves.at(START_VALVE);

    match best_path(start, &valves, 0, 0, vec![], 30) {
        Some(result) => result.pressure,
        None => panic!("got None from best_moves"),
    }
}

pub fn part_two(input: &str) -> usize {
    let valves = parse(input);
    let start = valves.at(START_VALVE);
    let time_allowed = 26;
    let human_paths = all_paths(start, &valves, 0, 0, vec![], time_allowed);
    let mut best_pressure = 0;

    for human_path in human_paths {
        let elephant_paths = all_paths(
            start,
            &valves,
            0,
            human_path.pressure,
            human_path.path,
            time_allowed,
        );
        for elephant_path in elephant_paths {
            best_pressure = best_pressure.max(elephant_path.pressure);
        }
    }

    best_pressure
}

fn best_path(
    current: &Valve,
    valves: &Valves,
    time_taken: usize,
    pressure: usize,
    path: Vec<String>,
    time_allowed: usize,
) -> Option<PathResult> {
    all_paths(current, valves, time_taken, pressure, path, time_allowed)
        .into_iter()
        .max()
}

fn all_paths(
    current: &Valve,
    valves: &Valves,
    time_taken: usize,
    pressure: usize,
    path: Vec<String>,
    time_allowed: usize,
) -> Vec<PathResult> {
    let mut out = vec![];

    // don't insert the start node as a path we travel to
    if time_taken != 0 {
        out.push(PathResult {
            time_taken,
            pressure,
            path: path.clone(),
        });
    }

    for (child_str, distance) in current.distances.iter() {
        let child = valves.at(child_str);
        let time_taken = time_taken + distance + 1;

        // if we've already traveled to this child previously, it doesn't make
        // sense to revisit as it's already on
        if path.contains(child_str) {
            continue;
        }

        // if we're going to exceed our max time, then we can't travel to this child
        if time_taken > time_allowed {
            continue;
        }

        let mut child_path = path.clone();
        child_path.push(child_str.to_string());

        let child_pressure = pressure + ((time_allowed - time_taken) * child.flow);
        out.extend(all_paths(
            child,
            valves,
            time_taken,
            child_pressure,
            child_path,
            time_allowed,
        ))
    }
    out
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
        assert_eq!(1651, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(2080, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(1707, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(2752, part_two(INPUT));
    }
}
