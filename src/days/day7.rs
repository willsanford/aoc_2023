use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand(cards: &[u8]) -> Hand {
    match cards {
        [5] => Hand::FiveOfAKind,
        [.., 4] => Hand::FourOfAKind,
        [3, 2] => Hand::FullHouse,
        [.., 3] => Hand::ThreeOfAKind,
        [.., 2, 2] => Hand::TwoPair,
        [.., 2] => Hand::OnePair,
        _ => Hand::HighCard,
    }
}

fn get_card_val(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

pub fn day7() {
    let filename = "data/day_7_ex.txt";
    let binding = read_to_string(filename).unwrap();
    let mut data: Vec<(&str, Hand, u64)> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let score: u64 = parts[1].parse().unwrap();

            let mut freq: Vec<u8> = parts[0]
                .chars()
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    acc
                })
                .iter()
                .map(|(_k, v)| *v)
                .collect();
            freq.sort();
            let hand = get_hand(&freq);
            (parts[0], hand, score)
        })
        .collect();

    data.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.chars()
                .map(get_card_val)
                .zip(b.0.chars().map(get_card_val))
                .find_map(|(a, b)| if a == b { None } else { Some(a.cmp(&b)) })
                .unwrap()
        } else {
            b.1.cmp(&a.1)
        }
    });

    let ans = data
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_cards, _, score))| {
            acc + (i + 1) as u64 * score
        });

    println!("{:?}", data);
    println!("{:?}", ans);
}
