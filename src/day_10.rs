// const INPUT: &str = include_str!("../inputs/day_10.txt");
//
// #[derive(Default, Debug)]
// struct Coordinate {
//     x:isize,
//     y:isize
// }
// const UP: Coordinate = Coordinate {x:0, y:1};
// const DOWN: Coordinate = Coordinate {x:0, y:-1};
// const LEFT: Coordinate = Coordinate {x:-1, y:0};
// const RIGHT: Coordinate = Coordinate {x:1, y:0};
//
// fn input_connections(input: &char) -> Connections {
//     match input {
//         '|' => Connections { up: true, left: false, right: false, down: true },
//         '-' => Connections { up: false, left: true, right: true, down: false },
//         'L' => Connections { up: true, left: false, right: true, down: false },
//         'J' => Connections { up: true, left: true, right: false, down: false },
//         '7' => Connections { up: false, left: true, right: false, down: true },
//         'F' => Connections { up: false, left: false, right: true, down: true },
//         '.' => Connections { up: false, left: false, right: false, down: true },
//         'S' => Connections { up: true, left: true, right: true, down: true },
//         c => panic!("Unknown pipe {c}")
//     }
// }
//
// pub fn star_1() {
//     let map = INPUT.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
//     let mut start: (usize, usize) = (0, 0);
//     'outer: for (r, row) in map.iter().enumerate() {
//         for (c, col) in row.iter().enumerate() {
//             if *col == 'S' {
//                 start = (r, c);
//                 break 'outer;
//             }
//         }
//     }
//     for
// }
//
// pub fn star_2() {}
