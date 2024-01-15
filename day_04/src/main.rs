use std::collections::HashSet;

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> u32 {
    let cards: Vec<Vec<HashSet<u32>>> = data
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .map(|c| {
            c.split(" | ")
                .map(|s| {
                    s.split_whitespace()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .collect()
        })
        .collect();
    cards
        .iter()
        .map(|g| {
            g[0].intersection(&g[1])
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

fn part_2(data: &str) -> usize {
    let cards: Vec<Vec<HashSet<u32>>> = data
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .map(|c| {
            c.split(" | ")
                .map(|s| {
                    s.split_whitespace()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .collect()
        })
        .collect();
    let mut counts = vec![1; cards.len()];
    for (i, game) in cards.iter().enumerate() {
        let cards_won = game[0].intersection(&game[1]).count() as u32;
        for x in 1..=cards_won {
            let j = i + x as usize;
            if j < counts.len() {
                counts[j] += counts[i] as usize;
            }
        }
    }
    counts.iter().sum::<usize>()
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_1(data), 13);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_2(data), 30);
    }
}
