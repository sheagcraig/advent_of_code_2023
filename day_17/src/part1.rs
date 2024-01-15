pub fn process(data: &str) -> i64 {
    let grid = day_17::parse(data);
    day_17::djikstra_min_heap(&grid, 1, 3)
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 102);
    }
}
