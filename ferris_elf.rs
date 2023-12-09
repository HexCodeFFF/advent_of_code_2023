use std::collections::HashMap;
pub fn run(INPUT: &str) -> impl std::fmt::Display {
    let mut lines = INPUT.lines();
    let steps: Vec<char> = lines.next().unwrap().chars().collect();
    let total_steps = steps.len();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_steps: Vec<&str> = vec![];
    for entry in lines.skip(1) {
        map.insert(&entry[0..3], (&entry[7..10], &entry[12..15]));
        if entry[0..3].ends_with('A') {
            current_steps.push(&entry[0..3]);
        }
    }
    let mut n_steps: Vec<usize> = vec![];
    for mut step in current_steps {
        let mut num_steps = 0;
        while !step.ends_with('Z') {
            // dbg!(&current_step);
            let next_step = map.get(step).unwrap();
            step = match steps[num_steps % total_steps] {
                'L' => next_step.0,
                'R' => next_step.1,
                s => {
                    panic!("Unknown letter {s}")
                }
            };
            num_steps += 1;
        }
        n_steps.push(num_steps);
    }
    let mut out = 1;
    for n in n_steps {
        // TODO: write it yourself you lazy bastard
        out = num::integer::lcm(out, n);
    }
    out
}
