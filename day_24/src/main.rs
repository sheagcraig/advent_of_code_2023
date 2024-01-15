mod part1;
mod part2;

fn main() {
    let data = include_str!("data.txt");
    let result = part1::process(data, 200000000000000.0..=400000000000000.0);
    println!("Part 1 Result: {result}");
    let result = part2::process(data);
    println!("Part 2 Result: {result}");
}

