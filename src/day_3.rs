const INPUT: &str = include_str!("../inputs/day_3.txt");
const LEN: usize = 140;

pub fn star_1() {
    let inp: Vec<Vec<char>> = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    for line in 0..LEN {
        let index = 0;
        for len in (1..4).rev() {
            if index + len < LEN {
                let numslice = &inp.get(line).unwrap()[index..index + len];
            }
        }
    }
}

pub fn star_2() {}
