use std::cmp::min;

const INPUT: &str = include_str!("../inputs/day_13.txt");

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let rows = v.len();
    let cols = v[0].len();
    (0..cols)
        .map(|col| (0..rows).map(|row| v[row][col]).collect())
        .collect()
}

pub fn star_1() {
    let map: Vec<Vec<Vec<bool>>> = INPUT
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().map(|ch| ch == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut summary = 0;
    for pattern in map {
        // horizontal
        // 123321
        //    ^ reflection index
        for reflection in 1..pattern.len() {
            let reflection_length = min(reflection.abs_diff(0), reflection.abs_diff(pattern.len()));
            // println!("{reflection} {reflection_length}")
            let left = pattern[reflection - reflection_length..reflection]
                .iter()
                .rev();
            let right = pattern[reflection..reflection + reflection_length].iter();
            if left.eq(right) {
                summary += reflection * 100;
                break;
            }
        }
        // vertical
        let transposed = transpose(pattern.clone());
        for reflection in 1..transposed.len() {
            let reflection_length = min(
                reflection.abs_diff(0),
                reflection.abs_diff(transposed.len()),
            );
            // println!("{reflection} {reflection_length}")
            // let left = pattern[reflection-reflection_length..reflection].iter().rev();
            let left = transposed[reflection - reflection_length..reflection]
                .iter()
                .rev();
            let right = transposed[reflection..reflection + reflection_length].iter();
            if left.eq(right) {
                summary += reflection;
                break;
            }
        }
    }
    println!("{summary}")
}

// #[derive(Eq, PartialEq)]
// struct Reflections {
//     vertical:Vec<usize>,
//     horizontal:Vec<usize>
// }
// const EMPTY_REFLECTION:Reflections = Reflections {vertical:vec!(), horizontal:vec!()};

fn count_reflection(
    pattern: &Vec<Vec<bool>>,
    ignore: (Option<usize>, Option<usize>),
) -> (Option<usize>, Option<usize>) {
    // let mut refs = EMPTY_REFLECTION;
    // horizontal
    // 123321
    //    ^ reflection index
    for reflection in 1..pattern.len() {
        if Some(reflection) == ignore.0 {
            continue;
        }
        let reflection_length = min(reflection.abs_diff(0), reflection.abs_diff(pattern.len()));
        // println!("{reflection} {reflection_length}")
        let left = pattern[reflection - reflection_length..reflection]
            .iter()
            .rev();
        let right = pattern[reflection..reflection + reflection_length].iter();
        if left.eq(right) {
            return (Some(reflection), None);
        }
    }
    // vertical
    let transposed = transpose(pattern.clone());
    for reflection in 1..transposed.len() {
        if Some(reflection) == ignore.1 {
            continue;
        }
        let reflection_length = min(
            reflection.abs_diff(0),
            reflection.abs_diff(transposed.len()),
        );
        // println!("{reflection} {reflection_length}")
        // let left = pattern[reflection-reflection_length..reflection].iter().rev();
        let left = transposed[reflection - reflection_length..reflection]
            .iter()
            .rev();
        let right = transposed[reflection..reflection + reflection_length].iter();
        if left.eq(right) {
            return (None, Some(reflection));
        }
    }
    (None, None)
}

pub fn star_2() {
    let map: Vec<Vec<Vec<bool>>> = INPUT
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().map(|ch| ch == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut summary = 0;
    'patternloop: for mut pattern in map {
        let original = count_reflection(&pattern, (None, None));
        for row in 0..pattern.len() {
            for col in 0..pattern[0].len() {
                // toggle bit
                pattern[row][col] ^= true;
                let desmudged_counted = count_reflection(&pattern, original);
                if let Some(row) = desmudged_counted.0 {
                    summary += row * 100;
                    continue 'patternloop;
                }
                if let Some(col) = desmudged_counted.1 {
                    summary += col;
                    continue 'patternloop;
                }
                // untoggle bit
                pattern[row][col] ^= true;
            }
        }
        for row in &pattern {
            for c in row {
                if *c {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        panic!("uh oh!")
    }
    println!("{summary}")
}
