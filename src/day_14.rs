const INPUT: &str = include_str!("../inputs/day_14.txt");

enum RockType {
    Rounded,
    Cube,
}
pub fn star_1() {
    let mut dish: Vec<Vec<Option<RockType>>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => None,
                    '#' => Some(RockType::Cube),
                    'O' => Some(RockType::Rounded),
                    r => panic!("unknown rock {r}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
}

pub fn star_2() {}
