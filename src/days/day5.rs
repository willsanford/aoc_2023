use std::fs::read_to_string;

pub fn day5() {
    let filename = "data/day_5_ex.txt";

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
    maps.sort_by(|a, b| a[1].cmp(&b[1]));

    /*
    // Ensure that all the starts of the maps are sorted
    let final_ranges: Vec<(u64, u64)> = maps.iter().fold(ranges, |r, map| {
        // For each iteration. we need to see if each part of the range gets fragmented by this map
        // and return the new fragmented ranges
        println!("{:?}", r);
        let updated_range: Vec<(u64, u64)> = Vec::new();
        let mut last_ind_used = 0;
        for (rstart, rlen) in map {}

        updated_range
    });
    */
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
