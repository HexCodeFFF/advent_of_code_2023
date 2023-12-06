use std::cmp::{max, min};

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

struct RangeOverlap {
    overlap: Option<(i64, i64)>,
    non_overlap_left: Option<(i64, i64)>,
    non_overlap_right: Option<(i64, i64)>,
}

fn ranges_overlap(r1: (i64, i64), r2: (i64, i64)) -> RangeOverlap {
    let (smaller_range, bigger_range) = {
        if r1.0 < r2.0 {
            (r1, r2)
        } else {
            (r2, r1)
        }
    };
    // hopefully where ranges overlap
    let overlap = (max(r1.0, r2.0), min(r1.1, r2.1));
    // check if it's valid
    let overlap_range = if overlap.0 < overlap.1 {
        Some(overlap)
    } else {
        None
    };
    let left: (i64, i64);
    let right: (i64, i64);
    if overlap_range.is_some() {
        // i did the math, these are the same regardless of if 1 is contained in another as long as there's no overlap
        left = (smaller_range.0, bigger_range.0);
        right = (smaller_range.1, bigger_range.1);
    } else {
        left = smaller_range;
        right = bigger_range;
    }
    RangeOverlap {
        overlap: overlap_range,
        non_overlap_left: {
            if left.0 == left.1 {
                None
            } else {
                Some(left)
            }
        },
        non_overlap_right: {
            if right.0 == right.1 {
                None
            } else {
                Some(right)
            }
        },
    }
}

pub fn star_2() {
    // each section
    let mut sections = INPUT.split("\n\n");
    let mut seeds_ranges = sections
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
        .collect::<Vec<i64>>()
        // chunk into 2s
        .chunks(2)
        // map to tuple
        .map(|s| (*s.iter().min().unwrap(), *s.iter().max().unwrap()))
        // collect
        .collect::<Vec<(i64, i64)>>();
    seeds_ranges.sort_by_key(|(a, b)| a);
    let mut new_seed_ranges: Vec<(i64, i64)> = vec![];
    for section in sections {
        let mut map: Vec<Mapping> = vec![];
        // first entry is label, skip and then iterate
        for entry in section.split('\n').skip(1) {
            // iterate over split by spaces to get 3 data points
            let mut nums = entry.split(' ');
            map.push(Mapping {
                destination_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                source_range_start: nums.next().unwrap().parse::<i64>().unwrap(),
                range_length: nums.next().unwrap().parse::<i64>().unwrap(),
            })
        }
        for seed in seeds_ranges.iter_mut() {
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
