use std::cmp::max;

pub fn star_1() {
    const INPUT: &str = include_str!("../inputs/day_2.txt");
    let mut sum = 0;
    // for each game
    for (mut id, game) in INPUT.lines().enumerate() {
        id += 1; // enumerate starts at 0
        let mut possible = true;
        for handful in game.split(": ").nth(1).unwrap().split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for col in handful.split(", ") {
                let colamount: Vec<&str> = col.split(' ').collect();
                let color = colamount[1];
                let amount = colamount[0].parse::<usize>().unwrap();

                match color {
                    "red" => {
                        red += amount;
                    }
                    "green" => {
                        green += amount;
                    }
                    "blue" => {
                        blue += amount;
                    }
                    invalid => {
                        panic!("Invalid color {invalid}")
                    }
                }
            }
            possible = red <= 12 && green <= 13 && blue <= 14;
            if !possible {
                break;
            }
        }
        if possible {
            sum += id;
        }
    }
    println!("{sum}");
}

pub fn star_2() {
    const INPUT: &str = include_str!("../inputs/day_2.txt");
    let mut power = 0;
    // for each game
    for (mut id, game) in INPUT.lines().enumerate() {
        id += 1; // enumerate starts at 0
        let mut game_red = 0;
        let mut game_green = 0;
        let mut game_blue = 0;
        for handful in game.split(": ").nth(1).unwrap().split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for col in handful.split(", ") {
                let colamount: Vec<&str> = col.split(' ').collect();
                let color = colamount[1];
                let amount = colamount[0].parse::<usize>().unwrap();

                match color {
                    "red" => {
                        red = amount;
                    }
                    "green" => {
                        green = amount;
                    }
                    "blue" => {
                        blue = amount;
                    }
                    invalid => {
                        panic!("Invalid color {invalid}")
                    }
                }
                game_red = max(game_red, red);
                game_green = max(game_green, green);
                game_blue = max(game_blue, blue);
            }
        }
        power += game_red * game_green * game_blue;
    }
    println!("{power}");
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::day_2::*;
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
