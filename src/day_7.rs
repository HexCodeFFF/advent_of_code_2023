use std::cmp::Reverse;
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day_7.txt");

fn count_items(list: &[char; 5]) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::with_capacity(13);
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

fn hand_value(hand: &[char; 5]) -> u8 {
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

fn full_hand_value(hand: &[char; 5]) -> u32 {
    // 4,3,2,1,0 = card value
    // h = hand value
    // _ = unused
    // ____ ____ hhhh 4444 3333 2222 1111 0000
    let mut out: u32 = 0;
    out |= card_value(&hand[4]) as u32;
    out |= (card_value(&hand[3]) as u32) << 4;
    out |= (card_value(&hand[2]) as u32) << 8;
    out |= (card_value(&hand[1]) as u32) << 12;
    out |= (card_value(&hand[0]) as u32) << 16;
    out |= (hand_value(hand) as u32) << 20;
    out
}

pub fn star_1() {
    let mut hands: Vec<([char; 5], usize, u32)> = INPUT
        .lines()
        .map(|l| {
            let lines: Vec<_> = l.split_ascii_whitespace().collect();
            let hand: [char; 5] = lines[0].chars().collect::<Vec<char>>().try_into().unwrap();
            (
                hand,
                lines[1].parse::<usize>().unwrap(),
                full_hand_value(&hand),
            )
        })
        .collect();
    hands.sort_by_key(|hand| hand.2);
    let mut winnings = 0;
    for (i, (_, bid, _)) in hands.iter().enumerate() {
        winnings += (i + 1) * bid;
    }
    // println!("{winnings}")
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

fn replace_in_vec(haystack: &[char; 5], needle: &char, replace: &char) -> [char; 5] {
    let mut out: [char; 5] = *haystack;
    for item in out.iter_mut() {
        if item == needle {
            *item = *replace
        }
    }
    out
}

fn hand_value2(hand: &[char; 5]) -> u8 {
    [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ]
    .iter()
    .map(|r| hand_value(&replace_in_vec(hand, &'J', r)))
    .max()
    .unwrap()
}

fn full_hand_value2(hand: &[char; 5]) -> u32 {
    // 4,3,2,1,0 = card value
    // h = hand value
    // _ = unused
    // ____ ____ hhhh 4444 3333 2222 1111 0000
    let mut out: u32 = 0;
    out |= card_value2(&hand[4]) as u32;
    out |= (card_value2(&hand[3]) as u32) << 4;
    out |= (card_value2(&hand[2]) as u32) << 8;
    out |= (card_value2(&hand[1]) as u32) << 12;
    out |= (card_value2(&hand[0]) as u32) << 16;
    out |= (hand_value2(hand) as u32) << 20;
    out
}

pub fn star_2() {
    let mut hands: Vec<([char; 5], usize, u32)> = INPUT
        .lines()
        .map(|l| {
            let lines: Vec<_> = l.split_ascii_whitespace().collect();
            let hand: [char; 5] = lines[0].chars().collect::<Vec<char>>().try_into().unwrap();
            (
                hand,
                lines[1].parse::<usize>().unwrap(),
                full_hand_value2(&hand),
            )
        })
        .collect();
    hands.sort_by_key(|hand| hand.2);
    let mut winnings = 0;
    for (i, (_, bid, _)) in hands.iter().enumerate() {
        winnings += (i + 1) * bid;
    }
    // println!("{winnings}")
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
