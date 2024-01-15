use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    XtremelyCoolLooking(u32),
    Musical(u32),
    Aerodynamic(u32),
    Shiny(u32)
}
#[derive(Debug)]
pub struct Rule {
    pub category: Option<Category>,
    pub cmp: Option<Ordering>,
    pub destination: String,
}

#[derive(Debug)]
pub struct Part {
    pub categories: HashMap<char, u32>
}

pub fn parse(data: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let (workflows, parts) = data.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Vec<Rule>> = workflows.lines()
        .map(|line| {
            let curly_brace = line.find('{').unwrap();
            let name = line[..curly_brace].to_string();
            let rules = line[curly_brace + 1..line.len() - 1].split(',')
                .map(|rule| {
                    // Handle the final rule in a workflow, which is just the name of the next workflow
                    if !rule.contains(':') {
                        return Rule { category: None, cmp: None, destination: rule.to_string() };
                    }
                    let (val, destination) = rule[2..].split_once(':').unwrap();
                    let val = val.parse::<u32>().unwrap();
                    let destination = destination.to_string();
                    let category = match &rule[0..1] {
                        "x" => Some(Category::XtremelyCoolLooking(val)),
                        "m" => Some(Category::Musical(val)),
                        "a" => Some(Category::Aerodynamic(val)),
                        "s" => Some(Category::Shiny(val)),
                        _ => panic!("Unknown category: {}", rule)
                    };
                    let cmp = match &rule[1..2] {
                        "<" => Some(Ordering::Less),
                        ">" => Some(Ordering::Greater),
                        _ => panic!("Unknown comparison: {}", rule)
                    };
                    Rule { category, cmp, destination }
                })
                .collect();
            (name, rules)
        }).collect();
    let parts: Vec<Part> = parts.lines()
        .map(|line| {
            Part { categories: line[1..line.len() - 1].split(',')
                .map(|category| {
                    let (cat, val) = category.split_once('=').unwrap();
                    let val = val.parse().unwrap();
                    (cat.chars().next().unwrap(), val)
                })
                .collect()
            }
        }).collect();
    (workflows, parts)
}