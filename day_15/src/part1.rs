pub fn process(data: &str) -> usize {
    data.split(',')
        .map(hash)
        .sum()
}

fn hash(data: &str) -> usize {
    data.chars()
        .map(|c| c as usize)
        .fold(0, |acc, c| (acc + c) * 17 % 256)
}
#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 1320);
    }
}
