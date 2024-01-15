use aoc::Point;

pub fn solve(instructions: &[(char, isize)]) -> isize {
    let path = get_edge(instructions);
    let trench_length = instructions.iter()
        .fold(0, |acc, (_, delta)| acc + delta);

    // Get the count of points inside the bounds of the above trench
    // using the shoelace formula
    let n = path.len();
    let mut trench_inner_area = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        trench_inner_area -= path[i].x * path[j].y;
        trench_inner_area += path[i].y * path[j].x;
    }
    trench_inner_area = trench_inner_area.abs() / 2;

    // I don't grasp this; I'm just copying it from the internet.
    // My assumption is that the shoelace formula above accounts for half of the
    // outside already?
    // The +1 is just due to zero-indexing
    (trench_length / 2) + trench_inner_area + 1
}

fn get_edge(instructions: &[(char, isize)]) -> Vec<Point> {
    let start = Point { x: 0, y: 0 };
    // Use scan to bring along an accumulator, which in this problem is
    // the current point location in the grid.
    let mut path: Vec<Point> = instructions.iter()
        .scan(start, |trench, (dir, delta)| {
            *trench += match dir {
                'U' => Point { x: 0, y: -*delta },
                'D' => Point { x: 0, y: *delta },
                'L' => Point { x: -*delta, y: 0 },
                'R' => Point { x: *delta, y: 0 },
                _ => panic!("Unknown direction"),
            };
            Some(*trench)
        })
        .collect();
    path.insert(0, start);
    path
}
