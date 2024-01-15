pub fn process(data: &str) -> isize {
    let instructions = parse(data);
    day_18::solve(&instructions)
}

pub fn parse(data: &str) -> Vec<(char, isize)> {
    data.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir = parts.next().unwrap().chars().next().unwrap();
            let delta: isize = parts.next().unwrap().parse().unwrap();
            (dir, delta)
        })
        .collect()
}

// I spent a lot of time trying to make this work, and I'm pretty upset it didn't.
// I'd like to circle back and do so.
// #[derive(PartialEq)]
// enum Status {
//     Outside,
//     // Bound,
//     Inside
// }

// fn get_grid(instructions: Vec<(char, isize, String)>, path: &Vec<Point>) -> Vec<Vec<char>> {
// // So we can figure out what the absolute min and max points are
//     let bounds = path.iter().fold((Point { x: 0, y: 0 }, Point { x: 0, y: 0 }), |(min, max), point| {
//         let min = Point {
//             x: min.x.min(point.x),
//             y: min.y.min(point.y),
//         };
//         let max = Point {
//             x: max.x.max(point.x),
//             y: max.y.max(point.y),
//         };
//         (min, max)
//     });
//     // Preallocate a grid with room to place this entire polygon within, with a
//     // padding of 1 so that we can easily determine inside/outside using the day 10 logic
//     // + 1 for exclusive range / off-by-one, +1 for padding each direction (e.g. top + 1 bottom + 1) == +3
//     let mut grid = vec![vec!['.'; (bounds.1.x - bounds.0.x + 3) as usize]; (bounds.1.y - bounds.0.y + 3) as usize];
//     let x_scale = 0 - bounds.0.x;
//     let y_scale = 0 - bounds.0.y;
//     let mut cur = Point { x: 1, y: 1 };
//     for instruction in instructions {
//         let delta = match instruction.0 {
//             'U' => UP,
//             'D' => DOWN,
//             'L' => LEFT,
//             'R' => RIGHT,
//             _ => panic!("Unknown direction"),
//         };
//         for _ in 0..instruction.1 {
//             cur += delta;
//             grid[(cur.y + y_scale) as usize][(cur.x + x_scale) as usize] = '#';
//         }
//     }
//     print_grid(&grid);
//     grid
// }
//
//
// fn print_grid(grid: &Vec<Vec<char>>) {
//     for row in grid {
//         for col in row {
//             print!("{}", col);
//         }
//         println!();
//     }
//     println!();
// }

// fn get_inside_count(grid: &Vec<Vec<char>>) -> usize {
//     for row in &grid {
//         let mut status = Status::Outside;
//         let mut in_edge = false;
//         for mut x in row {
//             if status == Status::Outside && x == &'#' && !in_edge {
//                 status = Status::Inside;
//                 in_edge = true;
//             }
//             if status == Status::Inside && x == &'#' && !in_edge {
//                 status = Status::Outside;
//                 in_edge = true;
//             }
//             if x == &'.' {
//                 in_edge = false;
//             }
//             if status == Status::Inside {
//                 x = &'#';
//             }
//         }
//     }
//     print_grid(&grid);
//
//     grid.iter().map(|row| row.iter().filter(|&col| col == &'#')).flatten().count()
//     0
// }

#[cfg(test)]
mod test {
    use crate::part1::*;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 62);
    }
}
