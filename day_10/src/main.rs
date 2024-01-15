use std::collections::{HashMap, HashSet};
use aoc::{Point, RIGHT, LEFT, UP, DOWN};

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> usize {
    struct Map {
        height: usize,
        width: usize,
        map: Vec<Vec<char>>,
    }

    impl Map {
        fn get_coords(&self, pt: Point) -> char {
            self.map[pt.y as usize][pt.x as usize]
        }
        fn find_next(&self, pt: Point, last: Option<Point>) -> Point {
            if last.is_none() {
                // We're at the start; any next will do.
                return self.find_connected(pt)[0];
            }
            let last = last.unwrap();
            let pipe_type = self.get_coords(pt);
            match pipe_type {
                'L' => {
                    if last.y < pt.y {
                        pt + RIGHT
                    } else {
                        pt + UP
                    }
                }
                'J' => {
                    if last.y < pt.y {
                        pt + LEFT
                    } else {
                        pt + UP
                    }
                }
                '7' => {
                    if last.y > pt.y {
                        pt + LEFT
                    } else {
                        pt + DOWN
                    }
                }
                'F' => {
                    if last.y > pt.y {
                        pt + RIGHT
                    } else {
                        pt + DOWN
                    }
                }
                '|' => {
                    if last.y > pt.y {
                        pt + UP
                    } else {
                        pt + DOWN
                    }
                }
                '-' => {
                    if last.x > pt.x {
                        pt + LEFT
                    } else {
                        pt + RIGHT
                    }
                }
                _ => panic!("Shit"),
            }
        }
        fn find_connected(&self, pt: Point) -> Vec<Point> {
            let mut next: Vec<Point> = vec![];
            if pt.x > 0 {
                let found = self.get_coords(pt + LEFT);
                if "-LF".contains(found) {
                    next.push(pt + LEFT)
                }
            };
            if (pt.x as usize) < self.width - 1 {
                let found = self.get_coords(pt + RIGHT);
                if "-J7".contains(found) {
                    next.push(pt + RIGHT)
                }
            };
            if pt.y > 0 {
                let found = self.get_coords(pt + UP);
                if "|7F".contains(found) {
                    next.push(pt + UP)
                }
            };
            if (pt.y as usize) < self.height - 1 {
                let found = self.get_coords(pt + DOWN);
                if "|JL".contains(found) {
                    next.push(pt + DOWN)
                }
            };
            next
        }
    }

    let (start, map) = parse(data);
    let map = Map {
        height: map.len(),
        width: map[0].len(),
        map,
    };
    let mut count = 1usize;
    let mut last = None;
    let mut next_forward = start;
    loop {
        let next_last = next_forward;
        next_forward = map.find_next(next_forward, last);
        count += 1;
        if next_forward == start {
            break;
        }
        last = Some(next_last);
    }
    count / 2
}

fn part_2(data: &str) -> i32 {
    // Start a new hashmap which tracks whether a point is outside or inside the pipe.
    // (HashMap<Point, bool>)
    // We can cheat and assume 0,0 is outside.
    // For each point, look L, R, U, D and if the neighbor is outside, then we're also outside.
    // The check is whether the point in question is already in the hashmap.
    // As a potential optimization, when we check each point, we can store all of the checked points
    // in a vec and once we arrive at a conclusion (we found a point that's outside or we ran out of
    // points to check), we can add the entire vec to the hashmap.
    // For the interior points, we have to check that they're truly enclosed. I think the trick here
    // is to do a similar thing to what we did for finding the first two pipe neighbors.
    // If you're looking down and the point is part of the pipe, if the char is ., |, J, or L (S?)
    // then you should keep looking down.
    // Otherwise, down is not going to lead to pipe escape!
    struct Map {
        height: usize,
        width: usize,
        map: Vec<Vec<char>>,
    }

    impl Map {
        fn get_coords(&self, pt: Point) -> char {
            self.map[pt.y as usize][pt.x as usize]
        }
        fn find_next(&self, pt: Point, last: Option<Point>) -> Point {
            if last.is_none() {
                // We're at the start; any next will do.
                return self.find_connected(pt)[0];
            }
            let last = last.unwrap();
            let pipe_type = self.get_coords(pt);
            match pipe_type {
                'L' => {
                    if last.y < pt.y {
                        pt + RIGHT
                    } else {
                        pt + UP
                    }
                }
                'J' => {
                    if last.y < pt.y {
                        pt + LEFT
                    } else {
                        pt + UP
                    }
                }
                '7' => {
                    if last.y > pt.y {
                        pt + LEFT
                    } else {
                        pt + DOWN
                    }
                }
                'F' => {
                    if last.y > pt.y {
                        pt + RIGHT
                    } else {
                        pt + DOWN
                    }
                }
                '|' => {
                    if last.y > pt.y {
                        pt + UP
                    } else {
                        pt + DOWN
                    }
                }
                '-' => {
                    if last.x > pt.x {
                        pt + LEFT
                    } else {
                        pt + RIGHT
                    }
                }
                _ => panic!("Shit"),
            }
        }
        fn find_connected(&self, pt: Point) -> Vec<Point> {
            let mut next: Vec<Point> = vec![];
            if pt.x > 0 {
                let found = self.get_coords(pt + LEFT);
                if "-LF".contains(found) {
                    next.push(pt + LEFT)
                }
            };
            if (pt.x as usize) < self.width - 1 {
                let found = self.get_coords(pt + RIGHT);
                if "-J7".contains(found) {
                    next.push(pt + RIGHT)
                }
            };
            if pt.y > 0 {
                let found = self.get_coords(pt + UP);
                if "|7F".contains(found) {
                    next.push(pt + UP)
                }
            };
            if (pt.y as usize) < self.height - 1 {
                let found = self.get_coords(pt + DOWN);
                if "|JL".contains(found) {
                    next.push(pt + DOWN)
                }
            };
            next
        }
    }
    let (start, map) = parse(data);
    let map = Map {
        height: map.len(),
        width: map[0].len(),
        map,
    };
    let mut pipe: HashMap<Point, char> = HashMap::new();
    pipe.insert(start, 'S');
    let mut last = None;
    let mut next_forward = start;
    loop {
        let next_last = next_forward;
        next_forward = map.find_next(next_forward, last);
        pipe.insert(next_forward, map.get_coords(next_forward));
        if next_forward == start {
            break;
        }
        last = Some(next_last);
    }
    let mut outside: HashSet<Point> = HashSet::new();
    let mut inside: HashSet<Point> = HashSet::new();

    #[derive(Debug, Eq, PartialEq)]
    enum Status {
        Outside,
        Inside,
    }
    for y in 0..map.height as isize {
        let mut status = Status::Outside;
        for x in 0..map.width as isize {
            let pt = Point { x, y };
            // If the point is not in the pipe, then it's outside or inside depending on the status.
            if !pipe.contains_key(&pt) {
                if status == Status::Outside {
                    outside.insert(pt);
                } else {
                    inside.insert(pt);
                }
            } else if match map.get_coords(pt) {
                // If it's a pipe section, then we need to count crossings, but only if they
                // are crossings in the direction we're iterating. So 7 is a crossing because we're
                // headed right, but J is not because we're sliding along the bottom of the J.
                '|' | '7' | 'F' | 'S' => true,
                _ => false,
            } {
                // Since we hit a pipe crossing, toggle the status.
                status = match status {
                    Status::Outside => Status::Inside,
                    Status::Inside => Status::Outside,
                }
            }
        }
    }
    inside.len() as i32
}

fn parse(data: &str) -> (Point, Vec<Vec<char>>) {
    let x_len = data.lines().next().unwrap().len();
    let start = data.replace('\n', "").find('S').unwrap();
    let start_coords = Point {
        x: (start % x_len) as isize,
        y: (start / x_len) as isize,
    };
    let chars = data.lines().map(|l| l.chars().collect()).collect();
    (start_coords, chars)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1_1() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_1(data), 4);
    }

    #[test]
    fn test_part_1_2() {
        let data = include_str!("sample_data_2.txt");
        assert_eq!(part_1(data), 8);
    }

    #[test]
    fn test_part_2_1() {
        let data = include_str!("sample_data_3.txt");
        assert_eq!(part_2(data), 4);
    }

    #[test]
    fn test_part_2_2() {
        let data = include_str!("sample_data_4.txt");
        assert_eq!(part_2(data), 4);
    }

    #[test]
    fn test_part_2_3() {
        let data = include_str!("sample_data_5.txt");
        assert_eq!(part_2(data), 8);
    }
}
