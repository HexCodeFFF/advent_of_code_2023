const INPUT: &str = include_str!("../inputs/day_4.txt");

pub fn star_1() {
    let mut sum = 0;
    for line in INPUT.lines() {
        let mut game_points = 0;
        let mut game = line.split(": ").nth(1).unwrap().split(" | ");
        let winning: Vec<&str> = game
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| !c.is_empty())
            .collect();
        let nums: Vec<&str> = game
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| !c.is_empty())
            .collect();
        for num in nums {
            if winning.contains(&num) {
                if game_points == 0 {
                    game_points = 1;
                } else {
                    game_points *= 2;
                }
            }
        }
        sum += game_points;
    }
    println!("{sum}")
}

pub fn star_2() {
    let lines: Vec<_> = INPUT.lines().collect();
    let mut total_cards = lines.len();
    let mut copies: Vec<usize> = vec![1; total_cards];
    for (line_number, line) in lines.iter().enumerate() {
        let mut game_points = 0;
        let num_of_copies = copies[line_number];
        let mut game = line.split(": ").nth(1).unwrap().split(" | ");
        let winning: Vec<&str> = game
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| !c.is_empty())
            .collect();
        let nums: Vec<&str> = game
            .next()
            .unwrap()
            .split(' ')
            .filter(|c| !c.is_empty())
            .collect();
        for num in nums {
            if winning.contains(&num) {
                game_points += 1;
            }
        }
        total_cards += num_of_copies * game_points;
        for i in line_number + 1..line_number + game_points + 1 {
            copies[i] += num_of_copies;
        }
    }
    println!("{total_cards}")
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_4::*;
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
