const INPUT: &str = include_str!("../inputs/day_3.txt");

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

pub fn star_1() {
    let mut sum = 0;
    let mut nums = 0;
    let mut numss: Vec<usize> = vec![];
    let num_of_lines = INPUT.lines().collect::<Vec<&str>>().len();
    for (line_index, line) in INPUT.lines().enumerate() {
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
                    if numslice.chars().all(|c| c.is_ascii_digit()) {
                        if let Ok(n) = numslice.parse::<usize>() {
                            // println!("num found: {n}");
                            num_at_pos = Some(n);
                            len_of_num = Some(len);
                            numss.push(n);
                            break;
                        }
                    }
                }
            }
            if let Some(n) = num_at_pos {
                nums += 1;
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
                        let offset_line = INPUT.lines().nth(offset_line_index as usize).unwrap();
                        println!("{offset_line}");
                        // for each row offset
                        for row_offset in -1..(l as isize) + 1 {
                            // find the absolute index
                            let offset_row_index = row_index as isize + row_offset;
                            // if offset is valid
                            if (0..line_len as isize).contains(&offset_row_index) {
                                // get the char
                                let offset_char =
                                    offset_line.chars().nth(offset_row_index as usize).unwrap();
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
    // dbg!(numss);
    println!("{nums}")
}

pub fn star_2() {}

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
