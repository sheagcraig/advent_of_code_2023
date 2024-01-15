use aoc::Point;
use std::collections::{HashSet, VecDeque};

fn get_grid(data: &str) -> (Vec<Vec<char>>, Point) {
    let grid = day_21::parse(data);
    let start = grid
        .iter()
        .enumerate()
        .fold(None, |acc, (i, row)| match acc {
            Some(_) => acc,
            None => match row.iter().position(|&c| c == 'S') {
                Some(x) => Some(Point {
                    x: x as isize,
                    y: i as isize,
                }),
                None => acc,
            },
        })
        .unwrap();
    (grid, start)
}

pub fn process(data: &str, steps: usize) -> usize {
    let (grid, start) = get_grid(data);

    // Magic!
    let solution_0 = bfs(&grid, start, steps.rem_euclid(grid.len()));
    let solution_1 = bfs(&grid, start, steps.rem_euclid(grid.len()) + grid.len());
    let solution_2 = bfs(
        &grid,
        start,
        steps.rem_euclid(grid.len()) + (grid.len() * 2),
    );
    let c = solution_0;
    let b = (4 * solution_1 - 3 * solution_0 - solution_2) / 2;
    let a = solution_1 - solution_0 - b;
    let x = (steps - grid.len() / 2) / grid.len();
    a * x.pow(2) + b * x + c
}

fn bfs(grid: &Vec<Vec<char>>, start: Point, steps: usize) -> usize {
    let mut queue = VecDeque::from([(Point { x: 0, y: 0 }, start, steps)]);

    let mut seen = HashSet::new();
    let mut answer = HashSet::new();

    let side_len = grid.len();

    while let Some((current_tile, current_loc, mut steps_remaining)) = queue.pop_front() {
        if steps_remaining % 2 == 0 {
            answer.insert((current_tile, current_loc));
        }

        if steps_remaining > 0 {
            steps_remaining -= 1;
            for mut neighbor in current_loc.neighbors() {
                let mut new_tile = current_tile;
                if neighbor.x < 0 {
                    new_tile = Point {
                        x: current_tile.x - 1,
                        y: current_tile.y,
                    };
                    neighbor = Point {
                        x: neighbor.x + side_len as isize,
                        y: neighbor.y,
                    }
                }
                if neighbor.x >= side_len as isize {
                    new_tile = Point {
                        x: current_tile.x + 1,
                        y: current_tile.y,
                    };
                    neighbor = Point {
                        x: neighbor.x - side_len as isize,
                        y: neighbor.y,
                    }
                }
                if neighbor.y < 0 {
                    new_tile = Point {
                        x: current_tile.x,
                        y: current_tile.y - 1,
                    };
                    neighbor = Point {
                        x: neighbor.x,
                        y: neighbor.y + side_len as isize,
                    }
                }
                if neighbor.y >= side_len as isize {
                    new_tile = Point {
                        x: current_tile.x,
                        y: current_tile.y + 1,
                    };
                    neighbor = Point {
                        x: neighbor.x,
                        y: neighbor.y - side_len as isize,
                    }
                }

                if seen.contains(&(new_tile, neighbor))
                    || grid[neighbor.y as usize][neighbor.x as usize] == '#'
                {
                    continue;
                }
                queue.push_back((new_tile, neighbor, steps_remaining));
                seen.insert((new_tile, neighbor));
            }
        }
    }
    answer.len()
}

#[cfg(test)]
mod test {
    use crate::part2::bfs;

    #[test]
    fn test_process_6() {
        let data = include_str!("sample_data_1.txt");
        let (grid, start) = super::get_grid(data);
        assert_eq!(bfs(&grid, start, 6), 16);
    }
    #[test]
    fn test_process_10() {
        let data = include_str!("sample_data_1.txt");
        let (grid, start) = super::get_grid(data);
        assert_eq!(bfs(&grid, start, 10), 50);
    }

    #[test]
    fn test_process_50() {
        let data = include_str!("sample_data_1.txt");
        let (grid, start) = super::get_grid(data);
        assert_eq!(bfs(&grid, start, 50), 1594);
    }

    #[test]
    fn test_process_100() {
        let data = include_str!("sample_data_1.txt");
        let (grid, start) = super::get_grid(data);
        assert_eq!(bfs(&grid, start, 100), 6536);
    }

    #[test]
    fn test_process_500() {
        let data = include_str!("sample_data_1.txt");
        let (grid, start) = super::get_grid(data);
        assert_eq!(bfs(&grid, start, 500), 167004);
    }

    #[test]
    fn test_process_1000() {
        let data = include_str!("sample_data_1.txt");
        let (grid, start) = super::get_grid(data);
        assert_eq!(bfs(&grid, start, 1000), 668697);
    }
    // #[test]
    // fn test_process_5000() {
    //     let data = include_str!("sample_data_1.txt");
    //     let (grid, start) = super::get_grid(data);
    //     assert_eq!(bfs(&grid, start, 5000), 16733044);
    // }
}
