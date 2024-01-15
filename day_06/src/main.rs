fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> u32 {
    let mut lines = data.lines().map(|line| {
        line.split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    let races = times.iter().zip(&distances);
    let mut results = Vec::new();
    for (&time, &distance) in races {
        let winners = (1..time)
            .filter(|charge_time| charge_time * (time - charge_time) > distance)
            .count() as u32;
        results.push(winners);
    }
    results.iter().product()
}

fn part_2(data: &str) -> u64 {
    let mut lines = data.lines().map(|line| {
        line.split(':')
            .nth(1)
            .unwrap()
            .replace(' ', "")
            .parse::<u64>()
            .unwrap()
    });
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    (1..time)
        .filter(|charge_time| charge_time * (time - charge_time) > distance)
        .count() as u64
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_1(data), 288);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_2(data), 71503);
    }
}
