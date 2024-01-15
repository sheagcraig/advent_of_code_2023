use itertools::{Itertools, repeat_n};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum State {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Debug)]
pub struct ConditionRecord {
    state: Vec<State>,
    damaged_springs: Vec<usize>,
}

fn generate_permutations(rec: &ConditionRecord) -> Vec<Vec<State>> {
    let unknowns = rec.state.iter().filter(|s| s == &&State::Unknown).count();
    repeat_n([State::Operational, State::Damaged].into_iter(), unknowns)
        .multi_cartesian_product()
        .collect()
}

fn count_valid(rec: &ConditionRecord, perms: Vec<Vec<State>>) -> usize {
    let mut count = 0;
    for perm in perms {
        let mut merged = vec![];
        let mut permuted_states = perm.iter();
        for s in rec.state.iter() {
            match s {
                State::Unknown => merged.push(permuted_states.next().unwrap()),
                _ => merged.push(s),
            }
        }
        if is_valid(merged, &rec.damaged_springs) {
            count += 1
        }
    }
    count
}

fn is_valid(merged: Vec<&State>, damaged_springs: &Vec<usize>) -> bool {
    let mut dmg_count: usize = 0;
    let mut dmg_springs = vec![];
    for s in merged.iter() {
        match s {
            State::Damaged => dmg_count += 1,
            _ => {
                if dmg_count > 0 {
                    dmg_springs.push(dmg_count);
                    dmg_count = 0;
                }
            }
        }
    }
    if dmg_count > 0 {
        dmg_springs.push(dmg_count);
    }
    &dmg_springs == damaged_springs
}

pub fn part_1(data: &str) -> usize {
    let condition_records = parse(data);
    let temps = condition_records
        .iter()
        .map(|rec| (rec, generate_permutations(rec)))
        .map(|(rec, perms)| count_valid(rec, perms))
        .sum();
    // From here, go through each record and try replacing the unknowns with the generated permutations
    // then check each resulting row to see if it is valid under the damaged_springs conditions.
    // Then sum the number of valid rows total.
    temps
}

#[cfg(test)]
mod test {
    use crate::part1::part_1;

    /// Test that the example data returns the correct result
    #[test]
    fn test_part_1_1() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_1(data), 21);
    }
}

pub fn parse(data: &str) -> Vec<ConditionRecord> {
    data.lines()
        .map(|l| {
            let state = l
                .split_whitespace()
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => State::Operational,
                    '#' => State::Damaged,
                    '?' => State::Unknown,
                    _ => panic!("Unexpected character in input: {}", c),
                })
                .collect();

            let damaged_springs = l
                .split_whitespace()
                .nth(1)
                .unwrap()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            ConditionRecord {
                state,
                damaged_springs,
            }
        })
        .collect()
}
