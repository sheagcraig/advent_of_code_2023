use std::collections::HashMap;

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> u32 {
    let mut lines = data.lines();
    let directions: Vec<_> = lines.next().unwrap().trim().chars().collect();
    let maps = lines
        .skip(1)
        .map(|line| {
            let mut iter = line.split(" = ");
            let key = iter.next().unwrap().to_string();
            let value = iter
                .next()
                .unwrap()
                .replace(['(', ')'], "")
                .split(", ")
                .map(str::to_string)
                .collect::<Vec<String>>();
            (key, value)
        })
        .collect::<HashMap<String, Vec<String>>>();

    let mut next = "AAA";
    for (i, dir) in directions.iter().cycle().enumerate() {
        if next == "ZZZ" {
            return i as u32;
        }
        if dir == &'L' {
            next = &maps[next][0];
        } else {
            next = &maps[next][1];
        }
    }
    0
}

fn part_2(data: &str) -> u64 {
    let mut lines = data.lines();
    let directions: Vec<_> = lines.next().unwrap().trim().chars().collect();
    let maps = lines
        .skip(1)
        .map(|line| {
            let mut iter = line.split(" = ");
            let key = iter.next().unwrap().to_string();
            let value = iter
                .next()
                .unwrap()
                .replace(['(', ')'], "")
                .split(", ")
                .map(str::to_string)
                .collect::<Vec<String>>();
            (key, value)
        })
        .collect::<HashMap<String, Vec<String>>>();

    let nexts = maps
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<&String>>();
    let cycles: Vec<u64> = nexts
        .iter()
        .map(|&m| {
            let mut next = m;
            for (i, dir) in directions.iter().cycle().enumerate() {
                if next.ends_with('Z') {
                    return i as u64;
                }
                if dir == &'L' {
                    next = &maps[next][0];
                } else {
                    next = &maps[next][1];
                }
            }
            0
        })
        .collect();
    let start = cycles[0];
    cycles[1..]
        .iter()
        .fold(start, |acc, x| num::integer::lcm(acc, *x))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1_1() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_1(data), 2);
    }

    #[test]
    fn test_part_1_2() {
        let data = include_str!("sample_data_2.txt");
        assert_eq!(part_1(data), 6);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data_3.txt");
        assert_eq!(part_2(data), 6);
    }
}
