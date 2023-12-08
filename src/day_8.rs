use num;
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day_8.txt");
pub fn star_1() {
    let mut lines = INPUT.lines();
    let steps: Vec<char> = lines.next().unwrap().chars().collect();
    let total_steps = steps.len();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for entry in lines.skip(1) {
        map.insert(&entry[0..3], (&entry[7..10], &entry[12..15]));
    }
    const START: &str = "AAA";
    const END: &str = "ZZZ";
    let mut num_steps = 0;
    let mut current_step = START;
    while current_step != END {
        // dbg!(&current_step);
        let next_step = map.get(current_step).unwrap();
        current_step = match steps[num_steps % total_steps] {
            'L' => next_step.0,
            'R' => next_step.1,
            s => {
                panic!("Unknown letter {s}")
            }
        };
        num_steps += 1;
    }
    println!("{num_steps}")
}
pub fn star_2() {
    let mut lines = INPUT.lines();
    let steps: Vec<char> = lines.next().unwrap().chars().collect();
    let total_steps = steps.len();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_steps: Vec<&str> = vec![];
    for entry in lines.skip(1) {
        map.insert(&entry[0..3], (&entry[7..10], &entry[12..15]));
        if entry[0..3].ends_with('A') {
            current_steps.push(&entry[0..3]);
        }
    }
    let mut n_steps: Vec<usize> = vec![];
    for mut step in current_steps {
        let mut num_steps = 0;
        while !step.ends_with('Z') {
            // dbg!(&current_step);
            let next_step = map.get(step).unwrap();
            step = match steps[num_steps % total_steps] {
                'L' => next_step.0,
                'R' => next_step.1,
                s => {
                    panic!("Unknown letter {s}")
                }
            };
            num_steps += 1;
        }
        n_steps.push(num_steps);
    }
    let mut out = 1;
    for n in n_steps {
        // TODO: write it yourself you lazy bastard
        out = num::integer::lcm(out, n);
    }
    println!("{}", out)
}
#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_8::*;
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
