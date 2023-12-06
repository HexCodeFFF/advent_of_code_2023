const INPUT: &str = include_str!("../inputs/day_5.txt");

#[derive(Debug)]
struct Mapping {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

pub fn star_1() {
    // each section
    let mut sections = INPUT.split("\n\n");
    let mut seeds = sections
        // first section
        .next()
        .unwrap()
        // split by space
        .split(' ')
        // skip first entry, which is the label
        .skip(1)
        // parse to nums
        .map(|s| s.parse::<i64>().unwrap())
        // collect
        .collect::<Vec<i64>>();
    for section in sections {
        let mut map: Vec<Mapping> = vec![];
        // first entry is label, skip and theniterate
        for entry in section.split('\n').skip(1) {
            // iterate over split by spaces to get 3 data points
            let mut nums = entry.split(' ');
            map.push(Mapping {
                destination_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                source_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                range_length: nums.next().unwrap().parse::<i64>().unwrap(),
            })
        }
        for seed in seeds.iter_mut() {
            for entry in map.iter() {
                // if seed is in the mapped range
                if (entry.source_range_start..entry.source_range_start + entry.range_length)
                    .contains(seed)
                {
                    // map the seed
                    *seed += entry.destination_range_start - entry.source_range_start;
                    break;
                }
            }
        }
    }
    println!("{}", seeds.iter().min().unwrap())
}

pub fn star_2() {
    // h
    // // each section
    // let mut sections = INPUT.split("\n\n");
    // let mut seeds = sections
    //     // first section
    //     .next()
    //     .unwrap()
    //     // split by space
    //     .split(' ')
    //     // skip first entry, which is the label
    //     .skip(1)
    //     // parse to nums
    //     .map(|s| s.parse::<i64>().unwrap())
    //     // collect
    //     .collect::<Vec<i64>>()
    //     // chunk into 2s
    //     .chunks(2)
    //     // map to tuple
    //     .map(|s| (s[0], s[1]))
    //     // collect
    //     .collect::<Vec<(i64,i64)>>();
    // for section in sections {
    //     let mut map: Vec<Mapping> = vec!();
    //     // first entry is label, skip and theniterate
    //     for entry in section.split('\n').skip(1) {
    //         // iterate over split by spaces to get 3 data points
    //         let mut nums = entry.split(' ');
    //         map.push(Mapping {
    //             destination_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
    //             source_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
    //             range_length: nums.next().unwrap().parse::<i64>().unwrap(),
    //         })
    //     }
    //     for seed in seeds.iter_mut() {
    //         for entry in map.iter() {
    //             // if seed is in the mapped range
    //             if (entry.source_range_start..entry.source_range_start + entry.range_length).contains(seed) {
    //                 // map the seed
    //                 *seed += entry.destination_range_start - entry.source_range_start;
    //                 break
    //             }
    //         }
    //     }
    // }
    // println!("{}", seeds.iter().min().unwrap())
}
