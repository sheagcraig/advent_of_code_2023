use std::collections::HashMap;

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> usize {
    let mut map = parse(data);
    tilt(&mut map, Direction::North);
    score(&map)
}

fn part_2(data: &str) -> usize {
    let cycled = cycle(parse(data), 1_000_000_000);
    score(&cycled)
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn cycle(map: Vec<Vec<char>>, cycles: usize) -> Vec<Vec<char>> {
    let mut tilted = map.clone();
    let mut cache = HashMap::<Vec<Vec<char>>, (usize, Vec<Vec<char>>)>::new();
    for i in 1..=cycles {
        if cache.contains_key(&tilted) {
            let cached = cache.get(&tilted).unwrap();
            tilted = cached.1.clone();
            let cycle_length = i - cached.0;
            if (cycles - cached.0) % cycle_length == 0 {
                return tilted;
            }
            continue;
        }
        let starting = tilted.clone();
        tilt(&mut tilted, Direction::North);
        tilt(&mut tilted, Direction::West);
        tilt(&mut tilted, Direction::South);
        tilt(&mut tilted, Direction::East);
        cache.insert(starting, (i, tilted.clone()));
    }
    tilted
}

fn tilt(tilted: &mut Vec<Vec<char>>, direction: Direction) {
    let x_len = tilted[0].len();
    let y_len = tilted.len();
    match direction {
        Direction::North => {
            for x in 0..x_len {
                for y in 1..y_len {
                    if tilted[y][x] == 'O' {
                        let mut final_y = y;
                        for rev_y in (0..y).rev() {
                            if tilted[rev_y][x] == '.' {
                                final_y = rev_y;
                            } else {
                                break;
                            }
                        }
                        if final_y < y {
                            tilted[y][x] = '.';
                            tilted[final_y][x] = 'O';
                        }
                    }
                }
            }
        }
        Direction::South => {
            for x in 0..x_len {
                for y in (0..y_len - 1).rev() {
                    if tilted[y][x] == 'O' {
                        let mut final_y = y;
                        for (rev_y, item) in tilted.iter().enumerate().take(y_len).skip(y + 1) {
                            if item[x] == '.' {
                                final_y = rev_y;
                            } else {
                                break;
                            }
                        }
                        if final_y > y {
                            tilted[y][x] = '.';
                            tilted[final_y][x] = 'O';
                        }
                    }
                }
            }
        }
        Direction::East => {
            for row in tilted {
                for x in (0..x_len - 1).rev() {
                    if row[x] == 'O' {
                        let mut final_x = x;
                        for (rev_x, item) in row.iter().enumerate().take(x_len).skip(x + 1) {
                            if item == &'.' {
                                final_x = rev_x;
                            } else {
                                break;
                            }
                        }
                        if final_x > x {
                            row[x] = '.';
                            row[final_x] = 'O';
                        }
                    }
                }
            }
        }
        Direction::West => {
            for row in tilted {
                for x in 1..x_len {
                    if row[x] == 'O' {
                        let mut final_x = x;
                        for rev_x in (0..x).rev() {
                            if row[rev_x] == '.' {
                                final_x = rev_x;
                            } else {
                                break;
                            }
                        }
                        if final_x < x {
                            row[x] = '.';
                            row[final_x] = 'O';
                        }
                    }
                }
            }
        }
    }
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

// fn print_map(map: &Vec<Vec<char>>) {
//     for row in map {
//         for c in row {
//             print!("{}", c);
//         }
//         println!();
//     }
// }

fn score(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, row)| (row.iter().filter(|&c| c == &'O').count()) * (map.len() - i))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Direction::*;

    #[test]
    /// Test scoring of part 1
    fn test_part_1() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_1(data), 136);
    }

    #[test]
    /// Test tilting of platform
    fn test_part_1_tilting() {
        let mut data = parse(include_str!("sample_data_1.txt"));
        let tilted = include_str!("sample_data_2.txt");
        tilt(&mut data, North);
        assert_eq!(data, parse(tilted));
    }

    #[test]
    fn test_part_2() {
        // Okay... Need to update `tilt` to work for any direction, preferably with a single loop. I'm not opposed to
        // just rotating the array if that's easier, although I suspect that all of the allocations and shuffling will
        // be painful vs. just doing it "right".
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_2(data), 64);
    }
    #[test]
    /// Test tilting of platform
    fn test_part_2_cycle() {
        let data = include_str!("sample_data_1.txt");
        let cycled = include_str!("sample_data_3.txt");
        assert_eq!(cycle(parse(data), 1), parse(cycled));
    }

    #[test]
    /// Test tilting of platform
    fn test_part_2_2cycle() {
        let data = include_str!("sample_data_1.txt");
        let cycled = include_str!("sample_data_2_2cycles.txt");
        assert_eq!(cycle(parse(data), 2), parse(cycled));
    }

    #[test]
    /// Test tilting of platform
    fn test_part_2_3cycle() {
        let data = include_str!("sample_data_1.txt");
        let cycled = include_str!("sample_data_2_3cycles.txt");
        assert_eq!(cycle(parse(data), 3), parse(cycled));
    }

    #[test]
    /// Test tilting of platform
    fn test_part_2_tilt_south() {
        let mut data = parse(include_str!("sample_data_1.txt"));
        let cycled = include_str!("sample_data_1_tilted_south.txt");
        tilt(&mut data, South);
        assert_eq!(data, parse(cycled));
    }

    #[test]
    /// Test tilting of platform
    fn test_part_2_tilt_east() {
        let mut data = parse(include_str!("sample_data_1.txt"));
        let cycled = include_str!("sample_data_1_tilted_east.txt");
        tilt(&mut data, East);
        assert_eq!(data, parse(cycled));
    }

    #[test]
    /// Test tilting of platform
    fn test_part_2_tilt_west() {
        let mut data = parse(include_str!("sample_data_1.txt"));
        let cycled = include_str!("sample_data_1_tilted_west.txt");
        tilt(&mut data, West);
        assert_eq!(data, parse(cycled));
    }
}
