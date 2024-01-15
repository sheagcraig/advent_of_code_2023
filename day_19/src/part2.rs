use std::collections::HashMap;
use day_19::*;

pub fn process(data: &str) -> usize {
    let (workflows, _) = parse(data);
    let ranges = std::array::from_fn(|_| (1..=4000).collect::<Vec<usize>>());
    solve("in", &workflows, ranges)
}

fn solve(name: &str, workflows: &HashMap<String, Vec<Rule>>, mut ranges: [Vec<usize>; 4]) -> usize {
    if name == "A" { return ranges.iter().map(|r| r.len()).product() }
    if name == "R" { return 0 }

    let mut result = 0;
    for rule in workflows.get(name).unwrap() {
        let mut new_ranges = ranges.clone();
        let (range_index, val) = match rule.category {
            Some(Category::XtremelyCoolLooking(v)) => (0, v),
            Some(Category::Musical(v)) => (1, v),
            Some(Category::Aerodynamic(v)) => (2, v),
            Some(Category::Shiny(v)) => (3, v),
            None => (5, 0),
        };
        if range_index != 5 {
            (new_ranges[range_index], ranges[range_index]) = ranges[range_index].iter().partition(|&x| (*x as u32).cmp(&val) == rule.cmp.unwrap());
        }
        result += solve(&rule.destination, workflows, new_ranges)
    }
    result
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 167409079868000);
    }
}
