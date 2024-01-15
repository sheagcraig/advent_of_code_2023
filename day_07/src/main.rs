use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let data = include_str!("data.txt");
    let result_1 = part_1(data);
    println!("Part 1 result: {result_1}");
    let result_2 = part_2(data);
    println!("Part 2 result: {result_2}");
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Card(u32);
impl Card {
    fn new(c: char) -> Self {
        Self(match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        })
    }
    fn new_jokers_wild(c: char) -> Self {
        Self(match c {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> u32 {
        let mut counts = HashMap::new();
        for card in self.cards.iter() {
            let count = counts.entry(card.0).or_insert(0);
            *count += 1;
        }
        if counts.len() == 1 {
            // Five of a kind, where all five cards have the same label: AAAAA
            6
        } else if counts.len() == 2 {
            // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
            if counts.values().any(|&count| count == 4) {
                5
            } else {
                // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
                4
            }
        } else if counts.len() == 3 {
            // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            if counts.values().any(|&count| count == 3) {
                3
            } else {
                // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
                2
            }
        } else {
            // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
            if counts.values().any(|&count| count == 2) {
                1
            } else {
                // High card, where the hand contains five cards with five distinct labels and does not qualify as any of the above hands: 23456
                0
            }
        }
    }
}

// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         match self.hand_type().partial_cmp(&other.hand_type()) {
//             Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
//             Some(ordering) => Some(ordering),
//             None => None,
//         }
//     }
// }
impl Ord for crate::Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialOrd for crate::Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_1(data: &str) -> u32 {
    let mut hands = data
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let cards: Vec<Card> = line.next().unwrap().chars().map(Card::new).collect();
            let bid = line.next().unwrap().parse::<u32>().unwrap();
            Hand { cards, bid }
        })
        .collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

#[derive(Eq, PartialEq, Debug)]
struct HandJokersWild {
    cards: Vec<Card>,
    bid: u32,
}

impl HandJokersWild {
    fn hand_type(&self) -> u32 {
        let mut counts = HashMap::new();
        for card in self.cards.iter() {
            let count = counts.entry(card.0).or_insert(0);
            *count += 1;
        }
        let jokers = counts.remove(&1).unwrap_or(0);
        if counts.values().max().unwrap_or(&0) + jokers == 5 {
            // Five of a kind, where all five cards have the same label: AAAAA
            6
        } else if counts.values().max().unwrap() + jokers == 4 {
            // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
            5
        } else if counts.len() == 2 || (counts.len() == 1 && jokers > 0) {
            // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
            4
        } else if counts.values().max().unwrap() + jokers == 3 {
            // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            3
        } else if counts.values().filter(|&count| count == &2).count() == 2 {
            // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
            // There's no way to have this low of a hand with jokers present
            assert!(jokers == 0);
            2
        } else if counts.values().any(|&count| count == 2) || jokers == 1 {
            // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
            1
        } else {
            // High card, where the hand contains five cards with five distinct labels and does not qualify as any of the above hands: 23456
            0
        }
    }
}

impl Ord for crate::HandJokersWild {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialOrd for crate::HandJokersWild {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_2(data: &str) -> u32 {
    let mut hands = data
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let cards: Vec<Card> = line
                .next()
                .unwrap()
                .chars()
                .map(Card::new_jokers_wild)
                .collect();
            let bid = line.next().unwrap().parse::<u32>().unwrap();
            HandJokersWild { cards, bid }
        })
        .collect::<Vec<HandJokersWild>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_part_1() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_1(data), 6440);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("sample_data.txt");
        assert_eq!(part_2(data), 5905);
    }
}
