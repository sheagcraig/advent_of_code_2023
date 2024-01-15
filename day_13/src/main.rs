fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> usize {
    let maps = parse(data);
    let mut h_reflections: Vec<usize> = vec![];
    let mut v_reflections: Vec<usize> = vec![];
    'outer: for map in maps.iter() {
        let result = get_reflection(map, None);
        match result {
            (Some(x), None) => {
                h_reflections.push(x);
                continue 'outer;
            }
            (None, Some(y)) => {
                v_reflections.push(y);
                continue 'outer;
            }
            _ => (),
        }
    }
    h_reflections.iter().sum::<usize>() + (v_reflections.iter().sum::<usize>() * 100)
}

fn part_2(data: &str) -> usize {
    let maps = parse(data);
    let mut h_reflections: Vec<usize> = vec![];
    let mut v_reflections: Vec<usize> = vec![];
    'outer: for og_map in maps.iter() {
        let og_result = get_reflection(og_map, None);
        for map in smudge_iter(og_map).into_iter() {
            let desmudged = get_reflection(&map, Some(og_result));
            if desmudged != og_result {
                match desmudged {
                    (Some(x), None) => {
                        h_reflections.push(x);
                        continue 'outer;
                    }
                    (None, Some(y)) => {
                        v_reflections.push(y);
                        continue 'outer;
                    }
                    _ => (),
                }
            }
        }
    }
    h_reflections.iter().sum::<usize>() + (v_reflections.iter().sum::<usize>() * 100)
}

fn get_reflection(
    map: &Vec<Vec<char>>,
    ignore: Option<(Option<usize>, Option<usize>)>,
) -> (Option<usize>, Option<usize>) {
    'inner: for y in 1..map.len() {
        if ignore.is_some() && ignore.unwrap().1.is_some() && ignore.unwrap().1.unwrap() == y {
            continue 'inner;
        }
        if map[y] == map[y - 1] {
            for m in 1..(map.len() - y) {
                if m + y < map.len() && y as isize - m as isize > 0 && map[y + m] != map[y - m - 1]
                {
                    continue 'inner;
                }
            }
            return (None, Some(y));
        }
    }
    let x_len = map[0].len();
    'inner_x: for x in 1..x_len {
        if ignore.is_some() && ignore.unwrap().0.is_some() && ignore.unwrap().0.unwrap() == x {
            continue 'inner_x;
        }
        if get_column(map, x) == get_column(map, x - 1) {
            for m in 1..(x_len - x) {
                if m + x < x_len
                    && x as isize - m as isize > 0
                    && get_column(map, x + m) != get_column(map, x - m - 1)
                {
                    continue 'inner_x;
                }
            }
            return (Some(x), None);
        }
    }
    (None, None)
}

fn get_column(map: &[Vec<char>], x: usize) -> Vec<char> {
    map.iter().map(|r| r[x]).collect()
}

fn smudge_iter(map: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let unsmudged = map.clone();
    // Create clones of the input, one for each position in the map, with that position's char toggled
    let mut results = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut smudged = unsmudged.clone();
            smudged[y][x] = if smudged[y][x] == '#' { '.' } else { '#' };
            results.push(smudged);
        }
    }
    results
}

fn parse(data: &str) -> Vec<Vec<Vec<char>>> {
    data.split("\n\n")
        .map(|m| m.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_1(data), 405);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_2(data), 400);
    }
    #[test]
    fn test_part_2_1() {
        let data = include_str!("sample_data_2.txt");
        assert_eq!(part_2(data), 4);
    }
}
