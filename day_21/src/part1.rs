use std::collections::HashSet;
use aoc::Point;

pub fn process(data: &str, steps: u32) -> usize {
    let grid = day_21::parse(data);
    let start = grid.iter()
        .enumerate()
        .fold(None, |acc, (i, row)| {
            match acc {
                Some(_) => acc,
                None => {
                    match row.iter()
                        .position(|&c| c == 'S') {
                            Some(x) => Some(Point{x: x as isize, y: i as isize}),
                            None => acc,
                        }
                }
            }
        }).unwrap();
    let mut reachable: HashSet<Point> = HashSet::from([start]);
    for _step in 0..steps {
        let mut next = HashSet::new();
        for point in reachable.iter() {
            for neighbor in point.neighbors() {
                if day_21::in_bounds(&grid, neighbor) && grid[neighbor.y as usize][neighbor.x as usize] != '#' {
                    next.insert(neighbor);
                }
            }
        }
        reachable = next;
    }
    reachable.len()
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data, 6), 16);
    }
}
