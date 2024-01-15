fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> i32 {
    let readings = parse(data);
    let mut results: Vec<i32> = Vec::new();
    for history in readings {
        let mut predictions = vec![history.clone()];
        loop {
            let mut next = Vec::new();
            for (i, n) in predictions.last().unwrap()[1..].iter().enumerate() {
                next.push(n - predictions.last().unwrap()[i]);
            }
            if next.iter().all(|&n| n == 0) {
                break;
            }
            predictions.push(next.clone());
        }

        for i in (1..predictions.len()).rev() {
            let prediction = predictions[i].last().unwrap() + predictions[i - 1].last().unwrap();
            predictions[i - 1].push(prediction);
        }
        results.push(*predictions[0].last().unwrap());
    }
    results.iter().sum()
}

fn part_2(data: &str) -> i32 {
    let readings = parse(data);
    let mut results: Vec<i32> = Vec::new();
    for history in readings {
        let mut predictions = vec![history.clone()];
        loop {
            let mut next = Vec::new();
            for (i, n) in predictions.last().unwrap()[1..].iter().enumerate() {
                next.push(n - predictions.last().unwrap()[i]);
            }
            if next.iter().all(|&n| n == 0) {
                break;
            }
            predictions.push(next.clone());
        }

        for i in (1..predictions.len()).rev() {
            let prediction = predictions[i - 1][0] - predictions[i][0];
            predictions[i - 1].insert(0, prediction);
        }
        results.push(predictions[0][0]);
    }
    results.iter().sum()
}

fn parse(data: &str) -> Vec<Vec<i32>> {
    data.lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_1(data), 114);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_2(data), 2);
    }
}
