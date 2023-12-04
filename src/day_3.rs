use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day_3.txt");

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

pub fn star_1() {
    let mut sum = 0;
    let input: Vec<Vec<char>> = INPUT
        // split into lines
        .lines()
        .collect::<Vec<&str>>()
        // split lines into chars
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let num_of_lines = input.len();
    for (line_index, line) in input.iter().enumerate() {
        // println!("{line}");
        let mut row_index = 0;
        let line_len = line.len();
        while row_index < line_len {
            let mut num_at_pos: Option<usize> = None;
            let mut len_of_num: Option<usize> = None;
            for len in (1..4).rev() {
                if row_index + len <= line_len {
                    let numslice = &line[row_index..row_index + len];
                    // println!("{numslice}");
                    if numslice.iter().all(|c| c.is_ascii_digit()) {
                        if let Ok(n) = numslice.iter().collect::<String>().parse::<usize>() {
                            // println!("num found: {n}");
                            num_at_pos = Some(n);
                            len_of_num = Some(len);
                            break;
                        }
                    }
                }
            }
            if let Some(n) = num_at_pos {
                let l = len_of_num.unwrap();
                let mut num_is_valid = false;
                // check all around the number
                // to make my life easier, this also checks the number itself, which isnt efficient but shouldnt affect the results
                // for each line offset
                for line_offset in -1isize..2isize {
                    // find the absolute index
                    let offset_line_index = line_index as isize + line_offset;
                    // if offset is valid
                    if (0..num_of_lines as isize).contains(&offset_line_index) {
                        // get the line
                        let offset_line = &input[offset_line_index as usize];
                        // for each row offset
                        for row_offset in -1..(l as isize) + 1 {
                            // find the absolute index
                            let offset_row_index = row_index as isize + row_offset;
                            // if offset is valid
                            if (0..line_len as isize).contains(&offset_row_index) {
                                // get the char
                                let offset_char = offset_line[offset_row_index as usize];
                                // if char is a symbol
                                num_is_valid = is_symbol(offset_char);
                                // break early to avoid extra computation
                                if num_is_valid {
                                    break;
                                }
                            }
                        }
                        // break early to avoid extra computation
                        if num_is_valid {
                            break;
                        }
                    }
                }
                // if line_index >= 1 {
                //     println!("{}\n{}\n{}", INPUT.lines().nth(line_index - 1).unwrap_or_default(), line, INPUT.lines().nth(line_index + 1).unwrap_or_default());
                // } else {
                //     println!("{}\n{}", line, INPUT.lines().nth(line_index + 1).unwrap_or_default());
                // }
                // for _ in 0..row_index {
                //     print!(" ")
                // }
                // println!("^ {n} {l} {num_is_valid}\n");

                if num_is_valid {
                    sum += n;
                }
                row_index += l;
            } else {
                row_index += 1;
            }
        }
    }
    println!("{sum}");
    sum;
}

pub fn star_2() {
    let input: Vec<Vec<char>> = INPUT
        // split into lines
        .lines()
        .collect::<Vec<&str>>()
        // split lines into chars
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let num_of_lines = input.len();
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (line_index, line) in input.iter().enumerate() {
        // println!("{line}");
        let mut row_index = 0;
        let line_len = line.len();
        while row_index < line_len {
            let mut num_at_pos: Option<usize> = None;
            let mut len_of_num: Option<usize> = None;
            for len in (1..4).rev() {
                if row_index + len <= line_len {
                    let numslice = &line[row_index..row_index + len];
                    // println!("{numslice}");
                    if numslice.iter().all(|c| c.is_ascii_digit()) {
                        if let Ok(n) = numslice.iter().collect::<String>().parse::<usize>() {
                            // println!("num found: {n}");
                            num_at_pos = Some(n);
                            len_of_num = Some(len);
                            break;
                        }
                    }
                }
            }
            if let Some(n) = num_at_pos {
                let l = len_of_num.unwrap();
                // check all around the number
                // to make my life easier, this also checks the number itself, which isnt efficient but shouldnt affect the results
                // for each line offset
                for line_offset in -1isize..2isize {
                    // find the absolute index
                    let offset_line_index = line_index as isize + line_offset;
                    // if offset is valid
                    if (0..num_of_lines as isize).contains(&offset_line_index) {
                        // get the line
                        let offset_line = &input[offset_line_index as usize];
                        // for each row offset
                        for row_offset in -1..(l as isize) + 1 {
                            // find the absolute index
                            let offset_row_index = row_index as isize + row_offset;
                            // if offset is valid
                            if (0..line_len as isize).contains(&offset_row_index) {
                                // get the char
                                let offset_char = offset_line[offset_row_index as usize];
                                // if char is a symbol
                                if offset_char == '*' {
                                    gears
                                        .entry((
                                            offset_line_index as usize,
                                            offset_row_index as usize,
                                        ))
                                        .or_default()
                                        .push(n);
                                }
                            }
                        }
                    }
                }
                // if line_index >= 1 {
                //     println!("{}\n{}\n{}", INPUT.lines().nth(line_index - 1).unwrap_or_default(), line, INPUT.lines().nth(line_index + 1).unwrap_or_default());
                // } else {
                //     println!("{}\n{}", line, INPUT.lines().nth(line_index + 1).unwrap_or_default());
                // }
                // for _ in 0..row_index {
                //     print!(" ")
                // }
                // println!("^ {n} {l} {num_is_valid}\n");

                row_index += l;
            } else {
                row_index += 1;
            }
        }
    }
    let mut sum = 0;
    for val in gears.into_values() {
        if val.len() == 2 {
            sum += val.get(0).unwrap() * val.get(1).unwrap()
        }
    }
    println!("{sum}")
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::day_3::*;
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
