use itertools::Itertools;
use std::cmp::{max, min};
use aoc::Point;

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    // Should have seen this coming...
    let result_2 = part_2(data, 1_000_000);
    println!("Part 2 result: {result_2}");
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum ImageKey {
    Galaxy,
    EmptySpace,
}


struct Map {
    galaxies: Vec<Point>,
    // x, y
    expansions: (Vec<usize>, Vec<usize>),
    expansion_size: usize,
}

// debugging aid; not updated to work with current expansion technique
// fn format_map(map: &Map) -> String {
//     let mut result = String::new();
//     for (i, row) in map.map.clone().into_iter().enumerate() {
//         for c in row {
//             match c {
//                 ImageKey::Galaxy => write!(&mut result, "#"),
//                 ImageKey::EmptySpace => write!(&mut result, "."),
//             };
//         }
//         if i < map.height - 1 {
//             write!(&mut result, "\n");
//         }
//     }
//     result
// }

fn parse_expand(data: &str, expansion: usize) -> Map {
    let map = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => ImageKey::EmptySpace,
                    '#' => ImageKey::Galaxy,
                    _ => panic!("Unknown character"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut extra_rows = vec![];
    for (y, _) in map.iter().enumerate() {
        if map[y].iter().all(|&c| c == ImageKey::EmptySpace) {
            extra_rows.push(y)
        }
    }
    let mut extra_columns = vec![];
    for x in 0..map[0].len() {
        if map.iter().all(|c| c[x] == ImageKey::EmptySpace) {
            extra_columns.push(x)
        }
    }
    let galaxies = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == ImageKey::Galaxy {
                    Some(Point {
                        x: x as isize,
                        y: y as isize,
                    })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    Map {
        galaxies,
        expansions: (extra_columns, extra_rows),
        expansion_size: expansion,
    }
}
fn part_1(data: &str) -> usize {
    let map = parse_expand(data, 2);
    // println!("{}", format_map(&map));
    // println!("{:?}", map.galaxies);
    map.galaxies
        .iter()
        .combinations(2)
        .map(|g| {
            let mut total = g[0].distance(*g[1]);
            total += (map.expansion_size - 1)
                * map
                    .expansions
                    .0
                    .iter()
                    .filter(|&&x| {
                        x > min(g[0].x, g[1].x) as usize && x < max(g[0].x, g[1].x) as usize
                    })
                    .count();
            total += (map.expansion_size - 1)
                * map
                    .expansions
                    .1
                    .iter()
                    .filter(|&&y| {
                        y > min(g[0].y, g[1].y) as usize && y < max(g[0].y, g[1].y) as usize
                    })
                    .count();
            total
        })
        .sum()
}

fn part_2(data: &str, expansion: usize) -> usize {
    let map = parse_expand(data, expansion);
    // println!("{}", crate::format_map(&map));
    // println!("{:?}", map.galaxies);
    map.galaxies
        .iter()
        .combinations(2)
        .map(|g| {
            let mut total = g[0].distance(*g[1]);
            total += (map.expansion_size - 1)
                * map
                    .expansions
                    .0
                    .iter()
                    .filter(|&&x| {
                        x > min(g[0].x, g[1].x) as usize && x < max(g[0].x, g[1].x) as usize
                    })
                    .count();
            total += (map.expansion_size - 1)
                * map
                    .expansions
                    .1
                    .iter()
                    .filter(|&&y| {
                        y > min(g[0].y, g[1].y) as usize && y < max(g[0].y, g[1].y) as usize
                    })
                    .count();
            total
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    /// Test that the example data returns the correct result
    #[test]
    fn test_part_1_1() {
        let data = include_str!("sample_data_1_1.txt");
        assert_eq!(part_1(data), 374);
    }

    /// Test that the data is parsed and expanded correctly
    // #[test]
    // fn test_part_1_2() {
    //     let data = include_str!("sample_data_1_1.txt");
    //     let expected= include_str!("sample_data_1_2.txt");
    //     assert_eq!(format_map(&parse_expand(data, 2)), expected);
    // }

    #[test]
    fn test_part_2_1() {
        let data = include_str!("sample_data_1_1.txt");
        assert_eq!(part_2(data, 10), 1030);
    }
    #[test]
    fn test_part_2_2() {
        let data = include_str!("sample_data_1_1.txt");
        assert_eq!(part_2(data, 100), 8410);
    }
}
