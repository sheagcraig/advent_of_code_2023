use std::collections::HashMap;
use std::collections::HashSet;

struct Candidate {
    chars: Vec<char>,
    part: bool,
}
struct Gear {
    chars: Vec<char>,
    gears: HashSet<(usize, usize)>,
}
impl Gear {
    fn new() -> Self {
        Self {
            chars: vec![],
            gears: HashSet::new(),
        }
    }

    fn add(&mut self, c: char) {
        self.chars.push(c);
    }

    fn part_number(&self) -> i32 {
        self.chars
            .clone()
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}

impl Candidate {
    fn new() -> Self {
        Self {
            chars: vec![],
            part: false,
        }
    }

    fn add(&mut self, c: char) {
        self.chars.push(c);
    }

    fn part_number(&self) -> i32 {
        self.chars
            .clone()
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn test_adjacent_star(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let l_bound = {
        if x == 0 {
            x
        } else {
            x - 1
        }
    };
    let r_bound = {
        if x == schematic[y].len() - 1 {
            x
        } else {
            x + 1
        }
    };
    let u_bound = {
        if y == 0 {
            y
        } else {
            y - 1
        }
    };
    let d_bound = {
        if y == schematic.len() - 1 {
            y
        } else {
            y + 1
        }
    };
    // println!("Checking for adjacent around {}, {} (bounds u{}, d{}, l{}, r{})", x, y, u_bound, d_bound, l_bound, r_bound);
    schematic[u_bound..=d_bound]
        .iter()
        .any(|n| n[l_bound..=r_bound].iter().any(|c| c == &'*'))
}
fn test_adjacent(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let l_bound = {
        if x == 0 {
            x
        } else {
            x - 1
        }
    };
    let r_bound = {
        if x == schematic[y].len() - 1 {
            x
        } else {
            x + 1
        }
    };
    let u_bound = {
        if y == 0 {
            y
        } else {
            y - 1
        }
    };
    let d_bound = {
        if y == schematic.len() - 1 {
            y
        } else {
            y + 1
        }
    };
    // println!("Checking for adjacent around {}, {} (bounds u{}, d{}, l{}, r{})", x, y, u_bound, d_bound, l_bound, r_bound);
    schematic[u_bound..=d_bound].iter().any(|n| {
        n[l_bound..=r_bound]
            .iter()
            .any(|c| !c.is_ascii_digit() && c != &'.')
    })
}
fn part_1(data: &str) -> i32 {
    let schematic: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut result = 0;
    for y in 0..schematic.len() {
        let mut candidate: Option<Candidate> = None;
        for x in 0..schematic[y].len() {
            if schematic[y][x].is_ascii_digit() {
                let mut c = match candidate {
                    None => Candidate::new(),
                    Some(c) => c,
                };
                c.add(schematic[y][x]);
                if test_adjacent(&schematic, x, y) {
                    c.part = true;
                }
                candidate = Some(c);
            } else {
                // Handle end of candidate
                // println!("End of candidate at {}, {}", x, y);
                if let Some(c) = candidate {
                    if c.part {
                        result += c.part_number();
                        // println!("Adding part number {}", c.part_number());
                    }
                }
                candidate = None;
            }
        }
        // Handle end of line
        // println!("End of line {}", y);
        if let Some(c) = candidate {
            if c.part {
                result += c.part_number();
                // println!("Adding part number {}", c.part_number());
            }
        }
    }
    // println!("Part 1: total part number sum: {}", result);
    result
}

fn part_2(data: &str) -> i32 {
    let schematic: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    // Build a map of star coord: vec of number
    // For each star, if it has len 2, product numbers and add to result
    let mut result = 0;
    let mut stars: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for y in 0..schematic.len() {
        let mut gear: Option<Gear> = None;
        for x in 0..schematic[y].len() {
            if schematic[y][x].is_ascii_digit() {
                let mut c = match gear {
                    None => Gear::new(),
                    Some(c) => c,
                };
                c.add(schematic[y][x]);
                if test_adjacent_star(&schematic, x, y) {
                    for coords in get_star_coords(&schematic, x, y) {
                        c.gears.insert(coords);
                    }
                }
                gear = Some(c);
            } else {
                // Handle end of candidate
                // println!("End of candidate at {}, {}", x, y);
                if let Some(c) = gear {
                    if !c.gears.is_empty() {
                        for (star_x, star_y) in c.gears.iter() {
                            stars
                                .entry((*star_x, *star_y))
                                .or_default()
                                .push(c.part_number());
                            // println!("Adding star number {}", c.part_number());
                        }
                        // match stars.entry(&(x, y)) {
                        //     Entry::Vacant(e) => { e.insert(vec![c.part_number()]); },
                        //     Entry::Occupied(mut e) => { e.get_mut().push(c.part_number()); },
                        // };
                        // result += c.part_number();
                    }
                }
                gear = None;
            }
        }
        // Handle end of line
        // println!("End of line {}", y);
        if let Some(c) = gear {
            if !c.gears.is_empty() {
                // stars[&(schematic[y].len(), y)].push(c.part_number());
                for (star_x, star_y) in c.gears.iter() {
                    stars
                        .entry((*star_x, *star_y))
                        .or_default()
                        .push(c.part_number());
                    // println!("Adding star number {}", c.part_number());
                }
                // result += c.part_number();
            }
        }
    }
    for v in stars.values() {
        if v.len() == 2 {
            result += v[0] * v[1];
        }
    }
    result
}

fn get_star_coords(schematic: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let l_bound = {
        if x == 0 {
            x
        } else {
            x - 1
        }
    };
    let r_bound = {
        if x == schematic[y].len() - 1 {
            x
        } else {
            x + 1
        }
    };
    let u_bound = {
        if y == 0 {
            y
        } else {
            y - 1
        }
    };
    let d_bound = {
        if y == schematic.len() - 1 {
            y
        } else {
            y + 1
        }
    };
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (star_y, row) in schematic.iter().enumerate().take(d_bound + 1).skip(u_bound) {
        for (star_x, item) in row.iter().enumerate().take(r_bound + 1).skip(l_bound) {
            if item == &'*' {
                result.push((star_x, star_y));
            }
        }
    }
    result
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_1(data), 4361);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_2(data), 467835);
    }
}
