use day_20::parse;

pub fn process(data: &str) -> usize {
    let mut setup =  parse(data);
    setup.rx_connected = "";
    let (low_count, high_count, _cycles) = day_20::solve(setup);
    low_count * high_count
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 32000000);
    }

    #[test]
    fn test_process_2() {
        let data = include_str!("sample_data_2.txt");
        assert_eq!(process(data), 11687500);
    }
}
