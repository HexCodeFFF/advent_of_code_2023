use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day_20.txt");

pub fn star_1() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::with_capacity(58);
    let mut init: Vec<&str>;
    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }
        let mut lne = line.split(" -> ");
        let input = lne.next().unwrap();
        let output = lne.next().unwrap();
        let out = output.split(", ").collect::<Vec<_>>();
        if input == "broadcaster" {
            init = out;
        } else {
            map.insert(input, out);
        }
    }
}

pub fn star_2() {}
