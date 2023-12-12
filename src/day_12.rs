const INPUT: &str = include_str!("../inputs/day_12.txt");

fn run_lens(run: &Vec<char>) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    let mut current_run: usize = 0;
    for char in run {
        if *char == '#' {
            current_run += 1;
        } else if current_run != 0 {
            out.push(current_run);
            current_run = 0;
        }
    }
    if current_run != 0 {
        out.push(current_run);
    }
    out
}

pub fn star_1() {
    let mut sum: u64 = 0;
    for line in INPUT.lines() {
        let mut l = line.split(' ');
        let pipes = l.next().unwrap();
        let lens = l
            .next()
            .unwrap()
            .split(',')
            .map(|r| r.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let chars = pipes.chars().collect::<Vec<_>>();
        let mut poses = Vec::with_capacity(20);
        for (i, char) in chars.iter().enumerate() {
            if *char == '?' {
                poses.push(i);
            }
        }
        for i in 0..2usize.pow(poses.len() as u32) {
            let mut mod_chars = chars.clone();
            for bit in 0..poses.len() {
                if i & (1 << bit) != 0 {
                    mod_chars[poses[bit]] = '#';
                } else {
                    mod_chars[poses[bit]] = '.';
                }
            }
            let runs = run_lens(&mod_chars);
            if runs == lens {
                sum += 1;
            }
            // for char in mod_chars.iter() {
            //     print!("{char}");
            // }
            // println!();
        }
    }
    println!("{sum}")
}

fn solve(input: &Vec<char>) -> usize {
    todo!()
}

pub fn star_2() {
    let mut sum: u64 = 0;
    for line in INPUT.lines() {
        let mut l = line.split(' ');
        let pipes = l.next().unwrap();
        let lens1 = l
            .next()
            .unwrap()
            .split(',')
            .map(|r| r.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut lens = Vec::with_capacity(lens1.len() * 5);
        for _ in 0..5 {
            lens.append(&mut lens1.clone())
        }
        let chars1 = pipes.chars().collect::<Vec<_>>();
        let mut chars = Vec::with_capacity(chars1.len() * 5);
        for _ in 0..5 {
            chars.append(&mut chars1.clone())
        }
        let mut poses = Vec::with_capacity(20);
        for (i, char) in chars.iter().enumerate() {
            if *char == '?' {
                poses.push(i);
            }
        }
    }
    println!("{sum}")
}
