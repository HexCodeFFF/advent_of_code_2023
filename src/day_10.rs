const INPUT: &str = include_str!("../inputs/day_10.txt");

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

const UP: Coordinate = Coordinate { x: 0, y: -1 };
const DOWN: Coordinate = Coordinate { x: 0, y: 1 };
const LEFT: Coordinate = Coordinate { x: -1, y: 0 };
const RIGHT: Coordinate = Coordinate { x: 1, y: 0 };

impl std::ops::Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

fn input_connections(input: &char) -> Vec<Coordinate> {
    match input {
        '|' => vec![UP, DOWN],
        '-' => vec![LEFT, RIGHT],
        'L' => vec![UP, RIGHT],
        'J' => vec![UP, LEFT],
        '7' => vec![LEFT, DOWN],
        'F' => vec![RIGHT, DOWN],
        '.' => vec![],
        'S' => vec![UP, DOWN, LEFT, RIGHT],
        c => panic!("Unknown pipe {c}"),
    }
}

fn flip_direction(input: &Coordinate) -> Coordinate {
    match input {
        &UP => DOWN,
        &DOWN => UP,
        &LEFT => RIGHT,
        &RIGHT => LEFT,
        u => panic!("Unknown coordinate {u:?}"),
    }
}

pub fn star_1() {
    let map = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut start = Coordinate::default();
    'outer: for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = Coordinate {
                    x: c as isize,
                    y: r as isize,
                };
                break 'outer;
            }
        }
    }
    let mut distances = vec![vec![-1; map[0].len()]; map.len()];
    for direction in [UP, DOWN, LEFT, RIGHT] {
        let mut position = start;
        // println!("{}", map[position.y as usize][position.x as usize]);
        distances[position.y as usize][position.x as usize] = 0;
        let mut last_direction = direction;
        position += direction;
        if !((0..map.len() as isize).contains(&position.y)
            && (0..map[0].len() as isize).contains(&position.x))
        {
            continue;
        }
        // this would be less code if it's inside the loop but i only need to check this once, the input is supposed to be valid loops
        let last_from_direction = flip_direction(&last_direction);
        let current_pipe = map[position.y as usize][position.x as usize];
        if !input_connections(&current_pipe).contains(&last_from_direction) {
            // println!("{direction:?} skipped");
            continue;
        }
        let mut distance = 1;
        // println!("going {direction:?}");
        loop {
            let last_from_direction = flip_direction(&last_direction);
            let current_pipe = map[position.y as usize][position.x as usize];
            // println!("{current_pipe} {position:?}");
            if current_pipe == 'S' {
                break;
            }
            let dist = &mut distances[position.y as usize][position.x as usize];
            match *dist {
                -1 => {
                    *dist = distance;
                }
                n => {
                    *dist = std::cmp::min(n, distance);
                }
            }
            let next_direction = input_connections(&current_pipe)
                .iter()
                .find(|d| **d != last_from_direction)
                .cloned()
                .unwrap();
            last_direction = next_direction;
            position += next_direction;
            distance += 1;
        }
    }
    // for row in &distances {
    //     for col in row {
    //         if *col == -1 {
    //             print!(".")
    //         } else {
    //             print!("{}", col % 10)
    //         }
    //
    //     }
    //     println!();
    // }
    let max_distance = distances.iter().flatten().max().unwrap();
    println!("{max_distance}");
}

// yes i can do this programmatically but shut up this is more efficient and easier
fn expand_pipe(input: &char) -> [[char; 3]; 3] {
    match input {
        '|' => [['.', '|', '.'], ['.', '|', '.'], ['.', '|', '.']],
        '-' => [['.', '.', '.'], ['-', '-', '-'], ['.', '.', '.']],
        'L' => [['.', '|', '.'], ['.', 'L', '-'], ['.', '.', '.']],
        'J' => [['.', '|', '.'], ['-', 'J', '.'], ['.', '.', '.']],
        '7' => [['.', '.', '.'], ['-', '7', '.'], ['.', '|', '.']],
        'F' => [['.', '.', '.'], ['.', 'F', '-'], ['.', '|', '.']],
        '.' => [['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']],
        'S' => [['.', '|', '.'], ['-', 'S', '-'], ['.', '|', '.']],
        c => panic!("Unknown pipe {c}"),
    }
}

fn flood_fill(canvas: &mut Vec<Vec<char>>, position: Coordinate) {
    // coord is valid
    let yrange = 0..canvas.len() as isize;
    let xrange = 0..canvas[0].len() as isize;
    let mut queue: Vec<Coordinate> = vec![position];
    while let Some(coord) = queue.pop() {
        if !(xrange.contains(&coord.x) && yrange.contains(&coord.y)) {
            continue;
        }
        let pos = &mut canvas[coord.y as usize][coord.x as usize];
        if *pos == '.' {
            *pos = 'X';
            for dir in [UP, DOWN, LEFT, RIGHT] {
                queue.push(coord + dir);
            }
        }
    }
}

pub fn star_2() {
    let map = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut start = Coordinate::default();
    'outer: for (r, row) in map.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = Coordinate {
                    x: c as isize,
                    y: r as isize,
                };
                break 'outer;
            }
        }
    }
    let mut cleaned_pipes = vec![vec!['.'; map[0].len()]; map.len()];
    for direction in [UP, DOWN, LEFT, RIGHT] {
        let mut position = start;
        // println!("{}", map[position.y as usize][position.x as usize]);
        cleaned_pipes[position.y as usize][position.x as usize] = 'S';
        let mut last_direction = direction;
        position += direction;
        if !((0..map.len() as isize).contains(&position.y)
            && (0..map[0].len() as isize).contains(&position.x))
        {
            continue;
        }
        // this would be less code if it's inside the loop but i only need to check this once, the input is supposed to be valid loops
        let last_from_direction = flip_direction(&last_direction);
        let current_pipe = map[position.y as usize][position.x as usize];
        if !input_connections(&current_pipe).contains(&last_from_direction) {
            // println!("{direction:?} skipped");
            continue;
        }
        // println!("going {direction:?}");
        loop {
            let last_from_direction = flip_direction(&last_direction);
            let current_pipe = map[position.y as usize][position.x as usize];
            // println!("{current_pipe} {position:?}");
            if current_pipe == 'S' {
                break;
            }
            let cleaned = &mut cleaned_pipes[position.y as usize][position.x as usize];
            match *cleaned {
                '.' => {
                    *cleaned = current_pipe;
                }
                _ => {
                    break;
                }
            }
            let next_direction = input_connections(&current_pipe)
                .iter()
                .find(|d| **d != last_from_direction)
                .cloned()
                .unwrap();
            last_direction = next_direction;
            position += next_direction;
        }
    }
    let mut expanded_pipes: Vec<Vec<char>> =
        vec![Vec::with_capacity(map[0].len() * 3); map.len() * 3];
    for (r, row) in cleaned_pipes.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            let expanded_pipe = expand_pipe(col);
            for i in 0..3 {
                expanded_pipes[r * 3 + i].extend_from_slice(&expanded_pipe[i])
            }
        }
    }
    flood_fill(&mut expanded_pipes, Coordinate { x: 0, y: 0 });
    let mut area: u64 = 0;
    for row in expanded_pipes.iter().skip(1).step_by(3) {
        for col in row.iter().skip(1).step_by(3) {
            area += (*col == '.') as u64
        }
    }
    println!("{area}")
    // for row in expanded_pipes.iter() {
    //     for (c,col) in row.iter().enumerate() {
    //         if c > 200 {
    //             break
    //         }
    //         print!("{col}")
    //     }
    //     println!();
    // }
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
