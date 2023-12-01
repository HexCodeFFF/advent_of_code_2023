pub fn star_1() {
    let input = std::fs::read_to_string("inputs/day_1.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                first_digit = Some(digit);
                break;
            }
        }
        for char in line.chars().rev() {
            if let Some(digit) = char.to_digit(10) {
                last_digit = Some(digit);
                break;
            }
        }
        // unwraps are safe assuming input is valid
        // probably faster than string parsing
        sum += first_digit.unwrap() * 10 + last_digit.unwrap();
    }
    println!("{sum}");
}

pub fn star_2() {
    let input = std::fs::read_to_string("inputs/day_1.txt").unwrap();
    let mut sum = 0;
    let num_names = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        let line_len = line.len();
        // for each character in line
        for i in 0..line_len {
            let mut digit: Option<u32> = None;
            // if character is number
            if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
                digit = Some(d);
            } else {
                // try each number string
                for (num, num_name) in num_names.iter().enumerate() {
                    // if number isnt too long at this location
                    if i + num_name.len() <= line_len {
                        // if string at this location matches number
                        if line[i..i + num_name.len()] == **num_name {
                            digit = Some(num as u32);
                        }
                    }
                }
            }
            // if we found a digit at this location
            if let Some(d) = digit {
                // only change if we havent found first digit
                if first_digit.is_none() {
                    first_digit = Some(d);
                }
                // last thing to modify this will be last digit
                last_digit = Some(d);
            }

        }
        // unwraps are safe assuming input is valid
        // probably faster than string parsing
        sum += first_digit.unwrap() * 10 + last_digit.unwrap();
    }
    println!("{sum}")
}