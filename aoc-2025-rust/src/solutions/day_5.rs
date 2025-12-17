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

pub fn part_2() -> u64 {
    let (id_ranges, _) = parse_input("../inputs/5.txt");

    let mut range_ends: Vec<RangeEnd> = vec![];

    for id_range in id_ranges {
        range_ends.push(RangeEnd {
            value: id_range.min,
            kind: RangeEndKind::Min,
        });

        range_ends.push(RangeEnd {
            value: id_range.max,
            kind: RangeEndKind::Max,
        });
    }

    range_ends.sort_by(|a, b| a.value.cmp(&b.value));

    let mut new_ranges: Vec<IdRange> = vec![];

    let mut currently_tracking_min: Option<u64> = None;
    let mut total_mins_visited = 0;
    let mut total_maxes_visited = 0;

    for range_end in range_ends {
        if matches!(range_end.kind, RangeEndKind::Min) {
            total_mins_visited += 1;
            if currently_tracking_min == None {
                currently_tracking_min = Some(range_end.value)
            }
        }

        if matches!(range_end.kind, RangeEndKind::Max) {
            total_maxes_visited += 1;

            if total_maxes_visited == total_mins_visited && currently_tracking_min.is_some() {
                let overlaps_previous_range = new_ranges.len() > 0
                    && currently_tracking_min.unwrap() == new_ranges.last().unwrap().max;

                if overlaps_previous_range {
                    let previous_most_recent_range = new_ranges.pop();
                    new_ranges.push(IdRange {
                        min: previous_most_recent_range.unwrap().min,
                        max: range_end.value,
                    })
                } else {
                    new_ranges.push(IdRange {
                        min: currently_tracking_min.unwrap(),
                        max: range_end.value,
                    });
                }

                currently_tracking_min = None;
            }
        }
    }

    let mut result: u64 = 0;

    for id_range in new_ranges {
        result += id_range.max - id_range.min + 1;
    }

    result
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

struct RangeEnd {
    value: u64,
    kind: RangeEndKind,
}

enum RangeEndKind {
    Min,
    Max,
}
