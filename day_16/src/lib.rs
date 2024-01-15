use std::collections::{HashSet, VecDeque};
use aoc::{Point, LEFT, RIGHT, UP, DOWN};

pub fn get_energization(note: &Vec<Vec<char>>, start_loc: Point, start_dir: Point) -> usize {
    let mut work = VecDeque::from(vec![(start_loc, start_dir)]);
    let mut energized: HashSet<(Point, Point)> = HashSet::new();
    while !work.is_empty() {
        let (mut beam_loc, mut beam_dir) = work.pop_front().unwrap();
        while crate::in_bounds(beam_loc, note) {
            if energized.contains(&(beam_loc, beam_dir)) {
                break;
            }
            energized.insert((beam_loc, beam_dir));
            match note[beam_loc.y as usize][beam_loc.x as usize] {
                '.' => {
                    beam_loc += beam_dir;
                    continue;
                },
                '/' => {
                    match beam_dir {
                        LEFT => beam_dir = DOWN,
                        RIGHT => beam_dir = UP,
                        UP => beam_dir = RIGHT,
                        DOWN => beam_dir = LEFT,
                        _ => panic!("Unexpected direction: {:?}", beam_dir),
                    }
                    beam_loc += beam_dir
                },
                '\\' => {
                    match beam_dir {
                        LEFT => beam_dir = UP,
                        RIGHT => beam_dir = DOWN,
                        UP => beam_dir = LEFT,
                        DOWN => beam_dir = RIGHT,
                        _ => panic!("Unexpected direction: {:?}", beam_dir),
                    }
                    beam_loc += beam_dir
                },
                '-' => {
                    // println!("Hit a splitter at {:?}", beam_loc);
                    match beam_dir {
                        LEFT | RIGHT => {},
                        UP | DOWN => {
                            work.push_back((beam_loc + LEFT, LEFT));
                            work.push_back((beam_loc + RIGHT, RIGHT));
                            // println!("Current deque {:?}", work);
                            break
                        },
                        _ => panic!("Unexpected direction: {:?}", beam_dir),
                    }
                    beam_loc += beam_dir
                },
                '|' => {
                    // println!("Hit a splitter at {:?}", beam_loc);
                    match beam_dir {
                        UP | DOWN => {},
                        LEFT | RIGHT => {
                            work.push_back((beam_loc + UP, UP));
                            work.push_back((beam_loc + DOWN, DOWN));
                            // println!("Current deque {:?}", work);
                            break
                        },
                        _ => panic!("Unexpected direction: {:?}", beam_dir),
                    }
                    beam_loc += beam_dir
                },
                _ => panic!("Unexpected character: {}", note[beam_loc.y as usize][beam_loc.x as usize]),
            }
        }
    }
    // print_energized(&note, &energized);
    energized.iter()
        .map(|c| c.0)
        .collect::<HashSet<Point>>()
        .len()
}

pub fn print_energized(note: &[Vec<char>], energized: &HashSet<(Point, Point)>) {
    for (y, line) in note.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if vec![UP, DOWN, RIGHT, LEFT].into_iter().any(|d| energized.contains(&(Point{x: x as isize, y: y as isize}, d))) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn in_bounds(point: Point, note: &Vec<Vec<char>>) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < note[0].len() as isize && point.y < note.len() as isize
}

pub fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}