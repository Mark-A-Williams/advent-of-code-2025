use std::collections::HashMap;

use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> i64 {
    let lines = get_lines_from_file("../inputs/2.txt");
    let ranges = parse_all_id_ranges(&lines.first().unwrap());

    return ranges
        .iter()
        .map(|o| o.get_sum_of_invalid_ids_in_range())
        .sum();
}

pub fn part_2() -> i64 {
    let lines = get_lines_from_file("../inputs/2.txt");
    let ranges = parse_all_id_ranges(&lines.first().unwrap());

    // for range in &ranges {
    //     println!("Range {0}", range.max - range.min)
    // }

    return ranges
        .iter()
        .map(|o| o.get_sum_of_invalid_ids_in_range_v2())
        .sum();
}

fn parse_all_id_ranges(raw_line: &String) -> Vec<IdRange> {
    raw_line.split(",").map(parse_id_range).collect()
}

fn parse_id_range(raw_line: &str) -> IdRange {
    let split: Vec<&str> = raw_line.split("-").collect();

    IdRange {
        min: split[0].parse::<i64>().unwrap(),
        max: split[1].parse::<i64>().unwrap(),
    }
}

struct IdRange {
    min: i64,
    max: i64,
}

impl IdRange {
    pub fn get_sum_of_invalid_ids_in_range(&self) -> i64 {
        let mut sum: i64 = 0;
        for id in self.min..self.max + 1 {
            let string = id.to_string();
            if string.len() % 2 != 0 {
                continue;
            }

            let split = string.split_at(string.len() / 2);
            if split.0 == split.1 {
                // println!("{id}");
                sum += id;
            }
        }

        return sum;
    }

    pub fn get_sum_of_invalid_ids_in_range_v2(&self) -> i64 {
        let mut sum: i64 = 0;
        for id in self.min..self.max + 1 {
            let string = id.to_string();
            if string_is_any_number_of_repeats_of_substring(&string) {
                sum += id
            }
        }

        return sum;
    }
}

fn string_is_any_number_of_repeats_of_substring(string: &str) -> bool {
    let len = string.len();

    if len == 1 {
        return false;
    }

    let factors = get_factors(len as i32);

    for factor in factors {
        let split = string.split_at(factor as usize).0;
        // println!(
        //     "Checking parent string '{0}' against substring '{1}' {2} times",
        //     string, split, factor
        // );

        let num_repeats = len as i32 / factor;
        if string_is_n_repeats_of_substring(string, split, num_repeats) {
            println!(
                "Match! '{0}' is {1} repeats of '{2}'",
                string, num_repeats, split
            );
            return true;
        }
    }

    return false;
}

fn string_is_n_repeats_of_substring(string: &str, substring: &str, n: i32) -> bool {
    let repeated = substring.repeat(n as usize);
    if repeated == string {
        return true;
    }

    // println!("Strings not equal: {0}, {1}", repeated, string);
    return false;
}

fn get_factors(number: i32) -> Vec<i32> {
    match number {
        4 => vec![1, 2],
        6 => vec![1, 2, 3],
        8 => vec![1, 2, 4],
        9 => vec![1, 3],
        10 => vec![1, 2, 5],
        _ => vec![1],
    }
}
