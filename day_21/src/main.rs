mod part1;
mod part2;

fn main() {
    let data = include_str!("data.txt");
    let result = part1::process(data, 64);
    println!("Part 1 Result: {result}");
    let result = part2::process(data, 26501365);
    println!("Part 2 Result: {:?}", result);
}

