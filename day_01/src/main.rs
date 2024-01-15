fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> u32 {
    data.lines()
        .map(|l| {
            format!(
                "{}{}",
                l.chars()
                    .nth(l.find(|c: char| c.is_ascii_digit()).unwrap())
                    .unwrap(),
                l.chars()
                    .nth(l.rfind(|c: char| c.is_ascii_digit()).unwrap())
                    .unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}

fn part_2(data: &str) -> u32 {
    data.lines()
        .map(|l| {
            let mut digits = Vec::<u32>::new();
            for i in 0..l.len() {
                match &l[i..] {
                    s if s.starts_with(|c: char| c.is_ascii_digit()) => {
                        digits.push(s.chars().next().unwrap().to_digit(10).unwrap())
                    }
                    s if s.starts_with("one") => digits.push(1),
                    s if s.starts_with("two") => digits.push(2),
                    s if s.starts_with("three") => digits.push(3),
                    s if s.starts_with("four") => digits.push(4),
                    s if s.starts_with("five") => digits.push(5),
                    s if s.starts_with("six") => digits.push(6),
                    s if s.starts_with("seven") => digits.push(7),
                    s if s.starts_with("eight") => digits.push(8),
                    s if s.starts_with("nine") => digits.push(9),
                    _ => continue,
                };
            }
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_1(data), 142);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data_2.txt");
        assert_eq!(part_2(data), 281);
    }
}
