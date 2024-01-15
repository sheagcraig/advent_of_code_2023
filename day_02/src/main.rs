#[derive(Debug)]
struct Game {
    number: u32,
    turns: Vec<Vec<Color>>,
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Game {
    fn new(game: &str) -> Game {
        let mut iter = game.split(": ");
        let number = iter
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let turns: Vec<Vec<Color>> = iter
            .next()
            .unwrap()
            .split("; ")
            .map(|t| {
                t.split(", ")
                    .map(|c| {
                        let mut iter = c.split_whitespace();
                        let count = iter.next().unwrap().parse().unwrap();
                        match iter.next().unwrap() {
                            "blue" => Color::Blue(count),
                            "red" => Color::Red(count),
                            "green" => Color::Green(count),
                            _ => panic!("Shouldn't get here"),
                        }
                    })
                    .collect()
            })
            .collect();
        Game { number, turns }
    }

    fn maxes(&self) -> (u32, u32, u32) {
        let mut max = (0, 0, 0);
        for turn in self.turns.iter() {
            for c in turn.iter() {
                match c {
                    Color::Red(n) => {
                        if *n > max.0 {
                            max.0 = *n;
                        };
                    }
                    Color::Green(n) => {
                        if *n > max.1 {
                            max.1 = *n;
                        };
                    }
                    Color::Blue(n) => {
                        if *n > max.2 {
                            max.2 = *n;
                        };
                    }
                }
            }
        }
        max
    }

    fn game_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        let maxes = self.maxes();
        red >= maxes.0 && green >= maxes.1 && blue >= maxes.2
    }
}

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

fn part_1(data: &str) -> u32 {
    let games: Vec<Game> = data.lines().map(Game::new).collect();
    games
        .iter()
        .filter_map(|g| {
            if g.game_possible(12, 13, 14) {
                Some(g.number)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(data: &str) -> u32 {
    let games: Vec<Game> = data.lines().map(Game::new).collect();
    games
        .iter()
        .map(|g| {
            let (r, g, b) = g.maxes();
            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_1(data), 8);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_2(data), 2286);
    }
}
