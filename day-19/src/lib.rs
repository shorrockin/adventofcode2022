use regex::Regex;

#[derive(Debug, Copy, Clone, Hash, Default, Eq, PartialEq)]
struct Blueprint {
    id: usize,
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
    max_ore_bots: usize,
    max_clay_bots: usize,
    max_obsidian_bots: usize,
}
impl Blueprint {
    fn from_input(input: &str) -> Vec<Blueprint> {
        let regex = Regex::new(r"\d+").unwrap();

        input
            .lines()
            .map(|line| {
                let numbers: Vec<usize> = regex
                    .captures_iter(line)
                    .map(|v| v[0].parse().unwrap())
                    .collect();
                Blueprint {
                    id: numbers[0],
                    ore_cost: Cost {
                        ore: numbers[1],
                        clay: 0,
                        obsidian: 0,
                    },
                    clay_cost: Cost {
                        ore: numbers[2],
                        clay: 0,
                        obsidian: 0,
                    },
                    obsidian_cost: Cost {
                        ore: numbers[3],
                        clay: numbers[4],
                        obsidian: 0,
                    },
                    geode_cost: Cost {
                        ore: numbers[5],
                        clay: 0,
                        obsidian: numbers[6],
                    },
                    max_ore_bots: numbers[1].max(numbers[2]).max(numbers[3]).max(numbers[5]),
                    max_clay_bots: numbers[4],
                    max_obsidian_bots: numbers[6],
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct RunState {
    blueprint: Blueprint,
    minute: u32,
    max_minutes: u32,
    ore_inventory: usize,
    clay_inventory: usize,
    obsidian_inventory: usize,
    geode_inventory: u32,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: u32,
}
impl RunState {
    fn new(blueprint: &Blueprint, max_minutes: u32) -> RunState {
        RunState {
            max_minutes,
            ore_bots: 1,
            blueprint: *blueprint,
            ..Default::default()
        }
    }

    // executes the current run state for the specified decision. if this
    // decision is not valid it will panic. returns the various decisions that
    // could be made at a choice intersection. returns none if we are complete.
    fn run(&mut self, mut decision: Decision, current_best: u32) -> Option<Vec<Decision>> {
        let ore_cost = self.blueprint.ore_cost;
        let clay_cost = self.blueprint.clay_cost;
        let obsidian_cost = self.blueprint.obsidian_cost;
        let geode_cost = self.blueprint.geode_cost;

        let mut skipped_ore_build = false;
        let mut skipped_clay_build = false;
        let mut skipped_obsidian_build = false;
        let mut skipped_geode_build = false;

        // if we made the decision to idle, then consider what we're not doing -
        // as it won't make sense to do it as our next decision.
        if decision == Decision::Idle {
            skipped_ore_build = self.can_afford(&ore_cost);
            skipped_clay_build = self.can_afford(&clay_cost);
            skipped_obsidian_build = self.can_afford(&obsidian_cost);
            skipped_geode_build = self.can_afford(&geode_cost);
        }

        loop {
            // if we're at time
            if self.minute == self.max_minutes {
                break;
            }

            // if, even if we built a bot geode bot every round, we couldn't
            // beat our max then short circuit.
            let turns_remaining = self.max_minutes - self.minute;
            let geode_max_growth = ((turns_remaining - 1) * turns_remaining) / 2;
            let max_potential =
                (self.geode_bots * turns_remaining) + geode_max_growth + self.geode_inventory;
            if max_potential <= current_best {
                break;
            }

            self.minute += 1;

            match decision {
                Decision::BuildOreBot => {
                    self.apply_cost(&ore_cost);
                    self.increment_inventory();
                    self.ore_bots += 1;
                }
                Decision::BuildClayBot => {
                    self.apply_cost(&clay_cost);
                    self.increment_inventory();
                    self.clay_bots += 1;
                }
                Decision::BuildObsidianBot => {
                    self.apply_cost(&obsidian_cost);
                    self.increment_inventory();
                    self.obsidian_bots += 1;
                }
                Decision::BuildGeodeBot => {
                    self.apply_cost(&geode_cost);
                    self.increment_inventory();
                    self.geode_bots += 1;
                }
                Decision::Idle => {
                    self.increment_inventory();
                }
            }

            // calculates what we can/should build. we should only consider
            // building something if we can both afford it, and we don't already
            // have the maximum amount that we need of that type. if you are
            // going to build something, you should also build it as soon as
            // possible. it does not make sense to be able to build something,
            // idle for a few turns, then build it later, thus don't consider
            // building something we've previously skipped.
            let build_ore_bot = !skipped_ore_build
                && self.ore_bots < self.blueprint.max_ore_bots
                && self.can_afford(&ore_cost);

            let build_clay_bot = !skipped_clay_build
                && self.clay_bots < self.blueprint.max_clay_bots
                && self.can_afford(&clay_cost);

            let build_obsidian_bot = !skipped_obsidian_build
                && self.obsidian_bots < self.blueprint.max_obsidian_bots
                && self.can_afford(&obsidian_cost);

            let build_geode_bot = !skipped_geode_build && self.can_afford(&geode_cost);

            match (
                build_ore_bot,
                build_clay_bot,
                build_obsidian_bot,
                build_geode_bot,
            ) {
                // can't build anything, continue the loop
                (false, false, false, false) => {
                    decision = Decision::Idle;
                }
                // else we have a decision to make, build out and return choices
                _ => {
                    let mut decisions = vec![];
                    if build_ore_bot {
                        decisions.push(Decision::BuildOreBot);
                    }
                    if build_clay_bot {
                        decisions.push(Decision::BuildClayBot);
                    }
                    if build_obsidian_bot {
                        decisions.push(Decision::BuildObsidianBot);
                    }
                    if build_geode_bot {
                        decisions.push(Decision::BuildGeodeBot);
                    }
                    decisions.push(Decision::Idle);

                    return Some(decisions);
                }
            }
        }

        None
    }

    fn increment_inventory(&mut self) {
        self.ore_inventory += self.ore_bots;
        self.clay_inventory += self.clay_bots;
        self.obsidian_inventory += self.obsidian_bots;
        self.geode_inventory += self.geode_bots;
    }

    fn apply_cost(&mut self, cost: &Cost) {
        self.ore_inventory -= cost.ore;
        self.clay_inventory -= cost.clay;
        self.obsidian_inventory -= cost.obsidian;
    }

    fn can_afford(&self, cost: &Cost) -> bool {
        self.ore_inventory >= cost.ore
            && self.clay_inventory >= cost.clay
            && self.obsidian_inventory >= cost.obsidian
    }
}

// as we iterate through the run state, at various points we can make the
// following decisions when we have enough resources to build something.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Decision {
    Idle,
    BuildOreBot,
    BuildClayBot,
    BuildObsidianBot,
    BuildGeodeBot,
}

pub fn part_one(input: &str) -> u32 {
    Blueprint::from_input(input)
        .iter()
        .map(|blueprint| calculate_max_geodes(blueprint, 24) * blueprint.id as u32)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    Blueprint::from_input(input)
        .iter()
        .take(3)
        .map(|blueprint| calculate_max_geodes(blueprint, 32))
        .product()
}

fn calculate_max_geodes(blueprint: &Blueprint, num_minutes: u32) -> u32 {
    // effectively maps our decision trees at the various run states that we
    // need track, will pop/push into this as we come across different decisions
    // that we need to make.
    let mut run_states = vec![(RunState::new(blueprint, num_minutes), Decision::Idle)];
    let mut max_geodes = 0;

    while let Some((mut run_state, decision)) = run_states.pop() {
        match run_state.run(decision, max_geodes) {
            Some(decisions) => {
                for next_decision in decisions {
                    run_states.push((run_state, next_decision));
                }
            }
            None => {
                max_geodes = max_geodes.max(run_state.geode_inventory);
            }
        }
    }
    max_geodes
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = include_str!("../input.example.txt");
    static INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_one_example() {
        assert_eq!(33, part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1349, part_one(INPUT));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(3472, part_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(21840, part_two(INPUT));
    }
}
