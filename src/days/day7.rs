use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Debug)]
enum Hand {
    HighCard([u8; 5]),
    OnePair([u8; 5]),
    TwoPair([u8; 5]),
    ThreeOfAKind([u8; 5]),
    FullHouse([u8; 5]),
    FourOfAKind([u8; 5]),
    FiveOfAKind([u8; 5]),
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare based on variant
        let variant_order = self.variant_index().cmp(&other.variant_index());
        if variant_order != Ordering::Equal {
            return variant_order;
        }

        // If variants are the same, compare the slices
        match (self, other) {
            (Hand::HighCard(a), Hand::HighCard(b))
            | (Hand::OnePair(a), Hand::OnePair(b))
            | (Hand::TwoPair(a), Hand::TwoPair(b))
            | (Hand::ThreeOfAKind(a), Hand::ThreeOfAKind(b))
            | (Hand::FullHouse(a), Hand::FullHouse(b))
            | (Hand::FourOfAKind(a), Hand::FourOfAKind(b))
            | (Hand::FiveOfAKind(a), Hand::FiveOfAKind(b)) => a.cmp(b),
            _ => unreachable!(),
        }
    }
}

impl Hand {
    fn variant_index(&self) -> u8 {
        match self {
            Hand::HighCard(_) => 0,
            Hand::OnePair(_) => 1,
            Hand::TwoPair(_) => 2,
            Hand::ThreeOfAKind(_) => 3,
            Hand::FullHouse(_) => 4,
            Hand::FourOfAKind(_) => 5,
            Hand::FiveOfAKind(_) => 6,
        }
    }
}

fn get_card_val(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1, // This is now the lowest card
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn get_hand(cards: &str) -> Hand {
    let s: Vec<u8> = cards.chars().map(get_card_val).collect();
    let s_slice: [u8; 5] = [s[0], s[1], s[2], s[3], s[4]];
    let mut freq_mat: HashMap<u8, u8> = s.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(*c).or_insert(0u8) += 1;
        acc
    });

    // If there is a joker, replace it with the most common card
    let num_jokers = *freq_mat.get(&1u8).unwrap_or(&0u8);
    if freq_mat.contains_key(&1u8) {
        freq_mat.remove(&1u8);
    }

    let mut freq: Vec<u8> = freq_mat.iter().map(|(_k, v)| *v).collect();
    freq.sort();
    if let Some(last) = freq.last_mut() {
        *last += num_jokers;
    } else {
        freq.push(num_jokers);
    }
    use Hand::*;
    match freq[..] {
        [5] => FiveOfAKind(s_slice),
        [.., 4] => FourOfAKind(s_slice),
        [2, 3] => FullHouse(s_slice),
        [.., 3] => ThreeOfAKind(s_slice),
        [.., 2, 2] => TwoPair(s_slice),
        [.., 2] => OnePair(s_slice),
        [.., 1] => HighCard(s_slice),
        _ => unreachable!(),
    }
}

pub fn day7() {
    let filename = "data/day_7.txt";
    let binding = read_to_string(filename).unwrap();
    let mut data: Vec<(Hand, u64)> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let score: u64 = parts[1].parse().unwrap();
            let hand = get_hand(&parts[0]);
            (hand, score)
        })
        .collect();

    data.sort();
    let ans = data
        .iter()
        .enumerate()
        .fold(0u64, |acc, (i, (_cards, score))| {
            acc + (i + 1) as u64 * score
        });

    println!("{:?}", ans);
}

// Part 1 Stuff
fn _get_card_val(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

// Part 1 Stuff
fn _get_hand(cards: &str) -> Hand {
    let s: Vec<u8> = cards.chars().map(_get_card_val).collect();
    let s_ref: [u8; 5] = [s[0], s[1], s[2], s[3], s[4]];
    let mut freq: Vec<u8> = s
        .iter()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
        .iter()
        .map(|(_k, v)| *v)
        .collect();
    freq.sort();
    match freq[..] {
        [5] => Hand::FiveOfAKind(s_ref),
        [.., 4] => Hand::FourOfAKind(s_ref),
        [2, 3] => Hand::FullHouse(s_ref),
        [.., 3] => Hand::ThreeOfAKind(s_ref),
        [.., 2, 2] => Hand::TwoPair(s_ref),
        [.., 2] => Hand::OnePair(s_ref),
        _ => Hand::HighCard(s_ref),
    }
}
pub fn _day7() {
    let filename = "data/day_7.txt";
    let binding = read_to_string(filename).unwrap();
    let mut data: Vec<(Hand, u64)> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let score: u64 = parts[1].parse().unwrap();
            let hand = _get_hand(&parts[0]);
            (hand, score)
        })
        .collect();

    data.sort();
    let ans = data
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_cards, score))| acc + (i + 1) as u64 * score);

    println!("{:?}", ans);
}
