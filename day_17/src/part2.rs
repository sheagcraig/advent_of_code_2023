use day_17::djikstra_min_heap;

pub fn process(data: &str) -> i64 {
    let grid = day_17::parse(data);
    djikstra_min_heap(&grid, 4, 10)
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 94);
    }

    #[test]
    fn test_process_sample_2() {
        let data = include_str!("sample_data_2.txt");
        assert_eq!(process(data), 71);
    }
}
