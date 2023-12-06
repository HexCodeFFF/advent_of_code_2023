const INPUT: &str = include_str!("../inputs/day_6.txt");
pub fn star_1() {
    let mut lines = INPUT.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<i64>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<i64>().unwrap());
    let mut total_ways = 1;
    for (time, distance) in times.zip(distances) {
        let mut num_of_beat_times: i64 = 0;
        for length_held in 0..time + 1 {
            if distance < length_held * (time - length_held) {
                num_of_beat_times += 1;
            }
        }
        total_ways *= num_of_beat_times;
    }
    println!("{total_ways}");
}

pub fn star_2() {
    let mut lines = INPUT.lines();
    let time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.to_string())
        .reduce(|a, b| format!("{a}{b}"))
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.to_string())
        .reduce(|a, b| format!("{a}{b}"))
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let mut num_of_beat_times: i64 = 0;
    for length_held in 0..time + 1 {
        if distance < length_held * (time - length_held) {
            num_of_beat_times += 1;
        }
    }
    println!("{num_of_beat_times}");
}
#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_6::*;
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
