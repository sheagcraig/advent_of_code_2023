mod part1;
mod part2;

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part1::part_1(data);
    println!("Part 1 result: {result_1}");
    // Should have seen this coming...
    let result_2 = part2::part_2(data, 5);
    println!("Part 2 result: {result_2}");
}
