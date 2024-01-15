use rayon::prelude::*;
use aoc::{DOWN, LEFT, Point, RIGHT, UP};
use day_16::get_energization;

pub fn process(data: &str) -> usize {
    let note = day_16::parse(data);
    let width = note[0].len();
    let height = note.len();
    let right_left_maxes = note.par_iter()
        .enumerate()
        .flat_map(|(y, line)|
            line.iter()
                .enumerate()
                .flat_map(|(_, _)| {
                    let right = get_energization(&note, Point {x: 0, y: y as isize}, RIGHT);
                    let left = get_energization(&note, Point {x: (width - 1) as isize, y: y as isize}, LEFT);
                    // dbg!(right, left);
                    [right, left]
                })
                .collect::<Vec<usize>>()
        )
        .max()
        .unwrap();
    let top_bottom_maxes = note[0].par_iter()
        .enumerate()
        .flat_map(|(x, _)| {
                let down = get_energization(&note, Point {x: x as isize, y: 0}, DOWN);
                let up= get_energization(&note, Point {x: x as isize, y: (height - 1) as isize}, UP);
                [down, up]
            })
        .max()
        .unwrap();
    std::cmp::max(right_left_maxes, top_bottom_maxes)
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 51);
    }
}
