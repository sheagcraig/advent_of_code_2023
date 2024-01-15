use pathfinding::matrix::{Matrix, directions};
use pathfinding::matrix::MatrixFormatError;


pub fn process(data: &str) -> usize {
    let grid = parse(data).expect("Should parse");
    // dbg!(grid.columns, grid.rows);
    // dbg!(&grid.get((grid.rows - 1, grid.columns - 2)));
    let paths: Vec<(Vec<(usize, usize)>, usize)> = pathfinding::directed::yen::yen(
        &(0, 1),
        |(y, x)| {
            grid.neighbours((*y, *x), false)
                .filter_map(|(ny, nx)| {
                    let dir = ((ny as isize - *y as isize), (nx as isize - *x as isize));
                    let next = grid.get((ny, nx)).unwrap();
                    // dbg!(ny, nx, dir, next);
                    if *next != '#' && match dir {
                        directions::N => "^.".contains(*next),
                        directions::S => "v.".contains(*next),
                        directions::E => ">.".contains(*next),
                        directions::W => "<.".contains(*next),
                        _ => false
                    } {
                        Some(((ny, nx), 1))
                    } else {
                        None
                    }
                }).collect::<Vec<((usize, usize), usize)>>()
        },
        |n| n == (&(grid.rows - 1, grid.columns - 2)),
        usize::MAX,
    );
    // dbg!(&paths);
    paths.iter().max_by_key(|(_, cost)| *cost).unwrap().1
}

fn parse(data: &str) -> Result<Matrix<char>, MatrixFormatError> {
    Matrix::from_rows(data.lines().map(|line| line.chars()))
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 94);
    }
}
