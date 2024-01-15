use std::collections::HashMap;
use itertools::Itertools;

pub fn part_2(data: &str, folds: u32) -> usize {
    let mut cache = HashMap::new();
    data.lines()
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            (left, right.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>())
        })
        .map(|(springs, counts)|
            (
                (0..folds).map(|_| springs).join("?"),
                (0..folds).flat_map(|_| &counts).copied().collect::<Vec<_>>()
            )
        )
        .map(|(springs, counts)| {
        cache.clear();
        solve(springs.as_bytes(), None, &counts, &mut cache)
    })
        .sum()
}

fn solve(springs: &[u8], current_sequence: Option<usize>, counts: &[usize], cache: &mut HashMap<(usize, usize, usize), usize>) -> usize {
    // Handle easy outcomes
    // End of a sequence; we're either completely done with finding our counts, or we're checking the last one.
    if springs.is_empty() {
        return match (current_sequence, counts.len()) {
            (None, 0) => 1,
            // if the sequence number is higher than the count, it doesn't match and can be ignored.
            (Some(x), 1) if x == counts[0] => 1,
            _ => 0,
        }
    }
    if current_sequence.is_some() && counts.is_empty() {
        return 0;
    }

    // Check in cache and return early on a hit
    let key = (springs.len(), current_sequence.unwrap_or(0), counts.len());
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    // Match on current_sequence and whether we're in the midst of a sequence and recurse.
    let result = match (springs[0], current_sequence) {
        // Sequence is over, see if we have a match for counts
        // no
        (b'.', Some(x)) if x != counts[0] => 0,
        // yes
        (b'.', Some(_)) => solve(&springs[1..], None, &counts[1..], cache),
        // Not a run of broken, so recurse from the next spring pos
        (b'.', None) => solve(&springs[1..], None, counts, cache),
        // We're in a run of broken, so recurse, increasing the sequence count
        (b'#', Some(x)) => solve(&springs[1..], Some(x + 1), counts, cache),
        // We're starting a new run of broken, so recurse, starting with a seq ct of 1
        (b'#', None) => solve(&springs[1..], Some(1), counts, cache),
        (b'?', Some(x)) => {
            // recurse with both options
            let mut subtotal = solve(&springs[1..], Some(x + 1), counts, cache);
            if x == counts[0] {
                subtotal += solve(&springs[1..], None, &counts[1..], cache)
            }
            subtotal
        },
        (b'?', None) => {
            // recurse with both options
            solve(&springs[1..], Some(1), counts, cache) +
            solve(&springs[1..], None, counts, cache)
        },
        _ => panic!("Invalid input"),
    };

    // cache what we've gotten
    cache.insert(key, result);

    result
}

#[cfg(test)]
mod test {
    use crate::part2::part_2;

    #[test]
    fn test_part_2_not_unfolded() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_2(data, 1), 21);
    }


    #[test]
    fn test_part_2_unfolded() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(part_2(data, 5), 525152);
    }
}
