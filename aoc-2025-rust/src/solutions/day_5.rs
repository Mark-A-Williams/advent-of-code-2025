use crate::solutions::file_helpers::get_lines_from_file;
use std::path::Path;

pub fn part_1() -> u32 {
    let (id_ranges, ids) = parse_input("../inputs/5.txt");

    let mut result = 0;

    for id in ids {
        if id_ranges
            .iter()
            .any(|range| range.max >= id && range.min <= id)
        {
            result += 1;
        }
    }

    result
}

pub fn part_2() -> u32 {
    0
}

fn parse_input(filename: impl AsRef<Path>) -> (Vec<IdRange>, Vec<u64>) {
    let mut id_ranges: Vec<IdRange> = vec![];
    let mut ids: Vec<u64> = vec![];

    let lines = get_lines_from_file(filename);

    for line in lines {
        if line.contains("-") {
            id_ranges.push(parse_id_range(&line))
        } else if line.is_empty() {
            continue;
        } else {
            ids.push(line.parse::<u64>().unwrap())
        }
    }

    (id_ranges, ids)
}

fn parse_id_range(raw_line: &str) -> IdRange {
    let split: Vec<&str> = raw_line.split("-").collect();

    IdRange {
        min: split[0].parse::<u64>().unwrap(),
        max: split[1].parse::<u64>().unwrap(),
    }
}

struct IdRange {
    min: u64,
    max: u64,
}
