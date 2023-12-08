use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
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
    values.sort();
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
    hands.sort_by(
        |hand1, hand2| compare_hands(&hand2.0, &hand1.0), /* reversed to sort descending */
    );
    dbg!(hands);
}

pub fn star_2() {}
