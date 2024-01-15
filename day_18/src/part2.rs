use std::primitive::i64;

pub fn process(data: &str) -> isize {
    let instructions = parse(data);
    day_18::solve(&instructions)
}

pub fn parse(data: &str) -> Vec<(char, isize)> {
    data.lines()
        .map(|line| {
            let color = &line.rsplit_once(' ').unwrap().1[2..8];
            let dir = match &color[5..] {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => panic!("Unknown direction"),
            };
            let delta= i64::from_str_radix(&color[..5], 16).unwrap() as isize;
            (dir, delta)
        })
        .collect()
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 952408144115);
    }
}
