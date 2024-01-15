use day_19::*;

pub fn process(data: &str) -> usize {
    let (workflows, parts) = day_19::parse(data);
    let mut accepted: Vec<Part> = vec![];
    let mut rejected: Vec<Part> = vec![];
    for part in parts {
        let mut next = "in";
        while !["A", "R"].contains(&next) {
            // dbg!(&next);
            let rules = &workflows[next];
            for rule in rules.iter() {
                match &rule.category {
                    Some(c) => {
                        let (tag, val) = match c {
                            Category::XtremelyCoolLooking(v) => ('x', v),
                            Category::Musical(v) => ('m', v),
                            Category::Aerodynamic(v) => ('a', v),
                            Category::Shiny(v) => ('s', v),
                        };
                        if part.categories.get(&tag).unwrap().cmp(val) == rule.cmp.unwrap() {
                            next = &rule.destination;
                            break
                        }
                    }
                    // This is a final rule, so use the destination without comparison
                    None => {
                        next = &rule.destination;
                        break
                    }
                }
            }
        }
        if next == "A" {
            accepted.push(part);
        } else {
            rejected.push(part);
        }
    }
    accepted.iter().map(|part| part.categories.values().sum::<u32>() as usize).sum()
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 19114);
    }
}
