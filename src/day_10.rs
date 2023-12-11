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

pub fn star_2() {}
