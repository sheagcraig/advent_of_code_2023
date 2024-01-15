use std::collections::BTreeMap;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct RangeMap {
    start: u64,
    end: u64,
    dest_start: u64,
}

impl RangeMap {
    fn convert(&self, n: u64) -> u64 {
        self.dest_start + (n - self.start)
    }
}

trait Convertor {
    fn to_next(&self, n: u64) -> u64;
}
type ElfMap = BTreeMap<u64, RangeMap>;

impl Convertor for ElfMap {
    fn to_next(&self, n: u64) -> u64 {
        let (_, max) = self.iter().next_back().unwrap();
        // Bail early rather than try all of the others
        if n > max.end {
            return n;
        }
        for (k, v) in self.iter() {
            if n >= *k && n < v.end {
                return v.convert(n);
            }
            // Bail early rather than try all of the others
            if n < *k {
                return n;
            }
        }
        n
    }
}

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> u64 {
    // Parse the first line of data into a vec of integers named seeds
    let seeds = data
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // println!("{:?}", seeds);
    // Parse data into a HashMap where the key is the header line and the value is a vec of lines of integers
    let data_map = data
        .split("\n\n")
        .skip(1)
        .collect::<Vec<&str>>()
        .iter()
        .map(|g| {
            let mut lines = g.lines();
            let _ = lines.next().unwrap().trim_end_matches(':');
            let ranges = lines
                .map(|l| {
                    l.split(' ')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .map(|r| RangeMap {
                    dest_start: r[0] as u64,
                    start: r[1] as u64,
                    end: (r[2] + r[1]) as u64,
                })
                .collect::<Vec<RangeMap>>();
            let mut result = ElfMap::new();
            for r in ranges {
                result.insert(r.start, r);
            }
            result
        })
        .collect::<Vec<ElfMap>>();
    // println!("{:?}", data_map);

    let mut results = vec![];
    for seed in &seeds {
        // println!("Seed: {}", seed);
        let mut next = *seed as u64;
        for elf in data_map.iter() {
            next = elf.to_next(next);
            // println!("{:?}", next);
        }
        results.push(next);
        // println!("Seed location: {}", next);
    }
    *results.iter().min().unwrap()
}

fn part_2(data: &str) -> u64 {
    let seeds = data
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let data_map = data
        .split("\n\n")
        .skip(1)
        .collect::<Vec<&str>>()
        .iter()
        .map(|g| {
            let mut lines = g.lines();
            let _ = lines.next().unwrap().trim_end_matches(':');
            let ranges = lines
                .map(|l| {
                    l.split(' ')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .map(|r| RangeMap {
                    dest_start: r[0] as u64,
                    start: r[1] as u64,
                    end: (r[2] + r[1]) as u64,
                })
                .collect::<Vec<RangeMap>>();
            let mut result = ElfMap::new();
            for r in ranges {
                result.insert(r.start, r);
            }
            result
        })
        .collect::<Vec<ElfMap>>();
    // println!("{:?}", data_map);

    let mut results: Vec<u64> = vec![];
    let seed_ranges = &seeds
        .chunks(2)
        .map(|c| (&c[0], &c[1]))
        .collect::<Vec<(&usize, &usize)>>();
    // Move each seed_range into a thread
    // Slap our BTreeMap into an ARC so we can share it
    let dm = Arc::new(data_map);
    let mut handles = Vec::new();

    for (&seed_start, &length) in seed_ranges {
        // println!("Seed range: {} - {}", seed_start, length);
        let dml = dm.clone();
        let handle = thread::spawn(move || {
            let mut results: Vec<u64> = vec![];
            for seed in seed_start..(seed_start + length) {
                let mut next = seed as u64;
                for elf in dml.iter() {
                    next = elf.to_next(next);
                }
                results.push(next);
            }
            results
        });
        handles.push(handle);
    }
    for handle in handles {
        results.push(*handle.join().unwrap().iter().min().unwrap());
    }
    // println!(
    //     "Part 2 lowest location: {:?}",
    //     results.iter().min().unwrap()
    // );
    *results.iter().min().unwrap()
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_1(data), 35);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_2(data), 46);
    }
}
