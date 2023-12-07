use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card character"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    stake: u32,
}

impl Hand {
    fn new(hand_str: &str) -> Self {
        let (hand_str, stake_str) = hand_str.split_once(' ').unwrap();
        let cards: Vec<_> = hand_str.chars().map(Card::from_char).collect();
        let hand_type = determine_hand_type(&cards);
        let stake = stake_str.trim().parse().unwrap();

        Hand {
            cards,
            hand_type,
            stake,
        }
    }
}

fn determine_hand_type(cards: &[Card]) -> HandType {
    generate_frequency_combinations(cards)
        .iter()
        .map(|freq| match freq.values().collect::<Vec<_>>().as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] | [1, 4] => HandType::FourOfAKind,
            [3, 2] | [2, 3] => HandType::FullHouse,
            [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 2, 1] | [2, 1, 2] | [1, 2, 2] => HandType::TwoPair,
            [2, 1, 1, 1] | [1, 2, 1, 1] | [1, 1, 2, 1] | [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid hand"),
        })
        .max()
        .unwrap()
}

fn generate_frequency_combinations(cards: &[Card]) -> Vec<HashMap<Card, usize>> {
    let mut combinations = vec![];
    let mut initial_freq = HashMap::new();
    let mut num_jokers = 0;

    cards.iter().for_each(|&card| {
        if card == Card::Joker {
            num_jokers += 1;
        } else {
            *initial_freq.entry(card).or_insert(0) += 1;
        }
    });

    if num_jokers == 0 {
        combinations.push(initial_freq);
    } else if num_jokers == 5 {
        let mut best_freq = HashMap::new();
        best_freq.insert(Card::Ace, 5);
        combinations.push(best_freq);
    } else {
        generate_joker_freq_combinations(&mut combinations, initial_freq, num_jokers);
    }

    combinations
}

fn generate_joker_freq_combinations(
    combinations: &mut Vec<HashMap<Card, usize>>,
    current_freq: HashMap<Card, usize>,
    num_jokers: usize,
) {
    if num_jokers == 0 {
        combinations.push(current_freq);
        return;
    }

    current_freq.keys().for_each(|&card_type| {
        let mut new_freq = current_freq.clone();
        *new_freq.entry(card_type).or_insert(0) += 1;
        generate_joker_freq_combinations(combinations, new_freq, num_jokers - 1);
    });
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

pub fn generator(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new).collect()
}

pub fn part_1(input: &[Hand]) -> u32 {
    let mut sorted_hands = input.to_vec();
    sorted_hands.sort_unstable();
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.stake)
        .sum()
}

pub fn part_2(input: &[Hand]) -> u32 {
    let mut sorted_hands = input
        .iter()
        .map(|hand| {
            let modified_cards = hand
                .cards
                .iter()
                .map(|&c| match c {
                    Card::Jack => Card::Joker,
                    _ => c,
                })
                .collect::<Vec<_>>();
            let hand_type = determine_hand_type(&modified_cards);
            Hand {
                cards: modified_cards,
                hand_type,
                stake: hand.stake,
            }
        })
        .collect::<Vec<_>>();
    sorted_hands.sort_unstable();
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.stake)
        .sum()
}
