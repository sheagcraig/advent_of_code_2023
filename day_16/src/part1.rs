use aoc::{Point, RIGHT};
use day_16::{parse, get_energization};

pub fn process(data: &str) -> usize {
    let note = parse(data);
    get_energization(&note, Point {x: 0, y: 0}, RIGHT)
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 46);
    }
}