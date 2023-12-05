use std::fs::read_to_string;

fn adjust_ranges(ranges: Vec<(u64, u64)>, maps: Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut adjusted_ranges = Vec::new();

    for (start, length) in ranges {
        let end = start + length;
        let mut current_start = start;

        for &(new_start, map_start, map_len) in &maps {
            let map_end = map_start + map_len;

            // Check if the current range overlaps with this mapping
            if current_start < map_end && end > map_start {
                // Adjust the start of the range to the new start based on the mapping
                if current_start >= map_start {
                    let new_range_start = new_start + (current_start - map_start);
                    let new_range_end = std::cmp::min(end, map_end);
                    let new_range_length = new_range_end - current_start;

                    adjusted_ranges.push((new_range_start, new_range_length));
                    current_start = new_range_end;
                } else {
                    // The part of the range before the mapping
                    adjusted_ranges.push((current_start, map_start - current_start));
                    current_start = map_start;
                }
            }
        }

        // Handle any remaining part of the range that wasn't covered by the mappings
        if current_start < end {
            adjusted_ranges.push((current_start, end - current_start));
        }
    }

    adjusted_ranges
}

pub fn day5() {
    let filename = "data/day_5.txt";

    let binding = read_to_string(filename).unwrap();
    let text: Vec<&str> = binding.trim().split("\n\n").collect();

    let needed_seeds: Vec<u64> = text[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for i in 0..needed_seeds.len() {
        if i % 2 == 0 {
            ranges.push((needed_seeds[i], needed_seeds[i + 1]));
        }
    }

    let mut maps: Vec<Vec<(u64, u64, u64)>> = text[1..]
        .iter()
        .map(|group| {
            group
                .split('\n')
                .filter(|line| !line.contains(':'))
                .map(|line| {
                    let vec: Vec<u64> =
                        line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                    (vec[0], vec[1], vec[2])
                })
                .collect()
        })
        .collect();
    // Sort the map by the start of the last key. We are guarenteed to not have overlapping maps
    for inner_map in maps.iter_mut() {
        inner_map.sort_by(|a, b| (*a).1.cmp(&(*b).1));
    }

    // Ensure that all the starts of the maps are sorted
    let final_ranges: u64 = maps
        .iter()
        .fold(ranges, |r, map| {
            // For each iteration. we need to see if each part of the range gets fragmented by this map
            adjust_ranges(r, map.clone())
        })
        .iter()
        .min_by_key(|(start, _length)| *start)
        .unwrap()
        .0;
    println!("{:?}", final_ranges);
}

pub fn _day5() {
    let filename = "data/day_5.txt";

    let binding = read_to_string(filename).unwrap();
    let text: Vec<&str> = binding.trim().split("\n\n").collect();

    let needed_seeds: Vec<u64> = text[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let maps: Vec<Vec<(u64, u64, u64)>> = text[1..]
        .iter()
        .map(|group| {
            group
                .split('\n')
                .filter(|line| !line.contains(':'))
                .map(|line| {
                    let vec: Vec<u64> =
                        line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                    (vec[0], vec[1], vec[2])
                })
                .collect()
        })
        .collect();

    let out: u64 = needed_seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |current, map| {
                let next_op = map.iter().find_map(|(end, start, offset)| {
                    if (current >= *start) && (current <= start + offset) {
                        Some(end + (current - start))
                    } else {
                        None
                    }
                });
                next_op.unwrap_or(current)
            })
        })
        .min()
        .unwrap();
    println!("{:?}", needed_seeds);
    println!("{:?}", out);
}
