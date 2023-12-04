use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    selected_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.split_once(':').unwrap();
        let id = card.strip_prefix("Card ").unwrap().trim().parse().unwrap();
        let (winning_numbers_s, selected_numbers_s) = numbers.split_once('|').unwrap();
        let winning_numbers = winning_numbers_s
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let selected_numbers = selected_numbers_s
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        Ok(Card {
            id,
            winning_numbers,
            selected_numbers,
        })
    }
}
pub fn generator(input: &str) -> Vec<Card> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|c| {
            let win_numbers = c
                .winning_numbers
                .iter()
                .filter(|n| c.selected_numbers.contains(n))
                .count();
            if win_numbers > 0 {
                2u32.pow((win_numbers as u32).saturating_sub(1))
            } else {
                0
            }
        })
        .sum()
}

pub fn part_2(input: &[Card]) -> u32 {
    let mut n_cards = vec![1; input.len()];

    input.iter().for_each(|c| {
        let matches = c.winning_numbers.iter().filter(|number| {
            c.selected_numbers.contains(number)
        }).count();

        let index = (c.id - 1) as usize;
        for i in 1..=matches {
            if index + i < input.len() {
                n_cards[index + i] += n_cards[index];
            }
        }
    });
    n_cards.iter().sum()
}
