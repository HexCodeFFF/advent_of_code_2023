use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::hash::Hash;

const INPUT: &str = include_str!("../inputs/day_7.txt");

fn count_items<T>(list: &Vec<T>) -> HashMap<T, usize>
where
    T: Hash + Eq + Sized + Copy,
{
    let mut map: HashMap<T, usize> = HashMap::with_capacity(13);
    for item in list {
        map.entry(*item).and_modify(|s| *s += 1).or_insert(1usize);
    }
    map
}

fn card_value(card: &char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c => {
            panic!("Unknown card {c}")
        }
    }
}

fn hand_value(hand: &Vec<char>) -> usize {
    let counted_hand = count_items(hand);
    let mut values = counted_hand.values().collect::<Vec<_>>();
    // doing a sort_by is faster than sort then reverse
    values.sort_by_key(|v| Reverse(*v));
    // this would probably look better with enums but meh too much code
    match values[0] {
        5 => 6, // five of a kind
        4 => 5, // four of a kind
        3 => {
            if *values[1] == 2 {
                4 // full house
            } else {
                3 // three of a kind
            }
        }
        2 => {
            if *values[1] == 2 {
                2 // two pair
            } else {
                1 // one pair
            }
        }
        1 => 0, // high card
        s => {
            panic!("Invalid card amount {s}")
        }
    }
}

fn compare_hands(hand1: &Vec<char>, hand2: &Vec<char>) -> Ordering {
    let value1 = hand_value(hand1);
    let value2 = hand_value(hand2);
    if value1 == value2 {
        for (card1, card2) in hand1.iter().zip(hand2.iter()) {
            if card1 != card2 {
                return card_value(card1).cmp(&card_value(card2));
            }
        }
        panic!("Hands {hand1:#?} and {hand2:#?} are identical in value!")
    } else {
        value1.cmp(&value2)
    }
}

pub fn star_1() {
    let mut hands: Vec<_> = INPUT
        .lines()
        .map(|l| {
            let lines: Vec<_> = l.split_ascii_whitespace().collect();
            (
                lines[0].chars().collect::<Vec<char>>(),
                lines[1].parse::<usize>().unwrap(),
            )
        })
        .collect();
    hands.sort_by(|hand1, hand2| compare_hands(&hand1.0, &hand2.0));
    let mut winnings = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        winnings += (i + 1) * bid;
    }
    println!("{winnings}")
}

fn card_value2(card: &char) -> u8 {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        c => {
            panic!("Unknown card {c}")
        }
    }
}

fn replace_in_vec(haystack: &Vec<char>, needle: &char, replace: &char) -> Vec<char> {
    let mut out: Vec<char> = Vec::with_capacity(haystack.len());
    for item in haystack {
        if item == needle {
            out.push(*replace)
        } else {
            out.push(*item);
        }
    }
    out
}

fn hand_value2(hand: &Vec<char>) -> usize {
    [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ]
    .iter()
    .map(|r| hand_value(&replace_in_vec(hand, &'J', r)))
    .max()
    .unwrap()
}

fn compare_hands2(hand1: &Vec<char>, hand2: &Vec<char>) -> Ordering {
    let value1 = hand_value2(hand1);
    let value2 = hand_value2(hand2);
    if value1 == value2 {
        for (card1, card2) in hand1.iter().zip(hand2.iter()) {
            if card1 != card2 {
                return card_value2(card1).cmp(&card_value2(card2));
            }
        }
        panic!("Hands {hand1:#?} and {hand2:#?} are identical in value!")
    } else {
        value1.cmp(&value2)
    }
}
pub fn star_2() {
    let mut hands: Vec<_> = INPUT
        .lines()
        .map(|l| {
            let lines: Vec<_> = l.split_ascii_whitespace().collect();
            (
                lines[0].chars().collect::<Vec<char>>(),
                lines[1].parse::<usize>().unwrap(),
            )
        })
        .collect();
    hands.sort_by(|hand1, hand2| compare_hands2(&hand1.0, &hand2.0));
    let mut winnings = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        winnings += (i + 1) * bid;
    }
    println!("{winnings}")
}
#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_7::*;
    use test::Bencher;

    #[bench]
    fn bench_star_2(b: &mut Bencher) {
        b.iter(star_2);
    }

    #[bench]
    fn bench_star_1(b: &mut Bencher) {
        b.iter(star_1);
    }
}
