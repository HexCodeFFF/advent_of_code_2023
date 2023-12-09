const INPUT: &str = include_str!("../inputs/day_9.txt");
pub fn star_1() {
    let mut sum: i64 = 0;
    for line in INPUT.lines() {
        let points: Vec<_> = line
            .split_ascii_whitespace()
            .map(|p| p.parse::<i64>().unwrap())
            .collect();
        let points_len = points.len();
        let mut points_and_derivates = vec![points];
        let mut max_derivative = 0;
        for nth_derivative in 1..points_len - 1 {
            let mut all_zeroes = true;
            let mut new_derivative: Vec<i64> = Vec::with_capacity(points_len - nth_derivative);
            for point in 0..points_len - nth_derivative {
                let d = points_and_derivates[nth_derivative - 1][point + 1]
                    - points_and_derivates[nth_derivative - 1][point];
                if d != 0 {
                    all_zeroes = false;
                }
                new_derivative.push(d);
            }
            points_and_derivates.push(new_derivative);
            if all_zeroes {
                max_derivative = nth_derivative;
                break;
            }
        }
        if max_derivative == 0 {
            panic!("No derivative is 0, unable to predict!")
        }
        let mut predicted_value: i64 = points_and_derivates[max_derivative - 1].last().unwrap()
            + points_and_derivates[max_derivative].last().unwrap();
        for nth_derivative in (1..max_derivative).rev() {
            predicted_value =
                points_and_derivates[nth_derivative - 1].last().unwrap() + predicted_value;
        }
        sum += predicted_value;
    }
    // println!("{sum}");
}
pub fn star_2() {
    let mut sum: i64 = 0;
    for line in INPUT.lines() {
        let points: Vec<_> = line
            .split_ascii_whitespace()
            .map(|p| p.parse::<i64>().unwrap())
            .collect();
        let points_len = points.len();
        let mut points_and_derivates = vec![points];
        let mut max_derivative = 0;
        for nth_derivative in 1..points_len - 1 {
            let mut all_zeroes = true;
            let mut new_derivative: Vec<i64> = Vec::with_capacity(points_len - nth_derivative);
            for point in 0..points_len - nth_derivative {
                let d = points_and_derivates[nth_derivative - 1][point + 1]
                    - points_and_derivates[nth_derivative - 1][point];
                if d != 0 {
                    all_zeroes = false;
                }
                new_derivative.push(d);
            }
            points_and_derivates.push(new_derivative);
            if all_zeroes {
                max_derivative = nth_derivative;
                break;
            }
        }
        if max_derivative == 0 {
            panic!("No derivative is 0, unable to predict!")
        }
        let mut predicted_value: i64 = points_and_derivates[max_derivative - 1].first().unwrap()
            - points_and_derivates[max_derivative].first().unwrap();
        for nth_derivative in (1..max_derivative).rev() {
            predicted_value =
                points_and_derivates[nth_derivative - 1].first().unwrap() - predicted_value;
        }
        sum += predicted_value;
    }
    // println!("{sum}");
}
#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_9::*;
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
