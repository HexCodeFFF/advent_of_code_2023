use std::cmp::{max, min};

const INPUT: &str = include_str!("../inputs/day_11.txt");
pub fn star_1() {
    let map = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut double_cols: Vec<bool> = vec![true; map[0].len()];
    let mut double_rows: Vec<bool> = vec![true; map.len()];
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.push((r, c));
            }
            double_cols[c] &= *col == '.';
            double_rows[r] &= *col == '.';
        }
    }
    // print!(" ");
    // for col in double_cols.iter() {
    //     if *col {
    //         print!("v")
    //     } else {
    //         print!(" ")
    //     }
    // }
    // println!();
    // for (r, row) in map.iter().enumerate() {
    //     if double_rows[r] {
    //         print!(">")
    //     } else{
    //         print!(" ")
    //     }
    //     for (c, col) in row.iter().enumerate() {
    //         print!("{col}")
    //     }
    //     println!();
    // }

    let mut sum = 0;
    // let mut galaxies_checked: Vec<(usize, usize)> = Vec::with_capacity(galaxies.len());
    for galaxy in galaxies.iter() {
        // galaxies_checked.push(*galaxy);
        for galaxy2 in galaxies.iter() {
            // if galaxies_checked.contains(galaxy2) {
            //     continue;
            // }
            let mut dist = 0;
            let maxx = max(galaxy.0, galaxy2.0);
            let maxy = max(galaxy.1, galaxy2.1);
            let minx = min(galaxy.0, galaxy2.0);
            let miny = min(galaxy.1, galaxy2.1);
            for x in minx..maxx {
                dist += 1 + double_rows[x] as usize
            }
            for y in miny..maxy {
                dist += 1 + double_cols[y] as usize
            }
            // dist += 1;
            // println!("{:?} {:?} {}", &galaxy, &galaxy2, dist);
            sum += dist;
        }
    }
    println!("{}", sum / 2)
}
pub fn star_2() {
    let map = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut double_cols: Vec<bool> = vec![true; map[0].len()];
    let mut double_rows: Vec<bool> = vec![true; map.len()];
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.push((r, c));
            }
            double_cols[c] &= *col == '.';
            double_rows[r] &= *col == '.';
        }
    }
    // print!(" ");
    // for col in double_cols.iter() {
    //     if *col {
    //         print!("v")
    //     } else {
    //         print!(" ")
    //     }
    // }
    // println!();
    // for (r, row) in map.iter().enumerate() {
    //     if double_rows[r] {
    //         print!(">")
    //     } else{
    //         print!(" ")
    //     }
    //     for (c, col) in row.iter().enumerate() {
    //         print!("{col}")
    //     }
    //     println!();
    // }

    let mut sum = 0;
    // let mut galaxies_checked: Vec<(usize, usize)> = Vec::with_capacity(galaxies.len());
    for galaxy in galaxies.iter() {
        // galaxies_checked.push(*galaxy);
        for galaxy2 in galaxies.iter() {
            // if galaxies_checked.contains(galaxy2) {
            //     continue;
            // }
            let mut dist = 0;
            let maxx = max(galaxy.0, galaxy2.0);
            let maxy = max(galaxy.1, galaxy2.1);
            let minx = min(galaxy.0, galaxy2.0);
            let miny = min(galaxy.1, galaxy2.1);
            for x in minx..maxx {
                dist += 1 + (double_rows[x] as usize * 999999)
            }
            for y in miny..maxy {
                dist += 1 + (double_cols[y] as usize * 999999)
            }
            // dist += 1;
            // println!("{:?} {:?} {}", &galaxy, &galaxy2, dist);
            sum += dist;
        }
    }
    println!("{}", sum / 2)
}
#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_11::*;
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
