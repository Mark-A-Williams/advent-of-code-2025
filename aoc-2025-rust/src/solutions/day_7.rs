use std::collections::HashMap;
use std::collections::HashSet;

use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u64 {
    let lines = get_lines_from_file("../inputs/7.txt");

    let mut previous_row_beam_locations: HashSet<usize> = HashSet::new();
    let mut split_count = 0;

    for line in lines.iter() {
        let mut current_row_beam_locations: HashSet<usize> = HashSet::new();

        for (index, char) in line.chars().enumerate() {
            if char == '.' {
                if previous_row_beam_locations.contains(&index) {
                    current_row_beam_locations.insert(index);
                    continue;
                }
            } else if char == '^' {
                if previous_row_beam_locations.contains(&index) {
                    current_row_beam_locations.insert(index + 1);
                    current_row_beam_locations.insert(index - 1);
                    split_count += 1;
                }
            } else if char == 'S' {
                current_row_beam_locations.insert(index);
            }
        }

        previous_row_beam_locations = current_row_beam_locations;
    }

    split_count
}

pub fn part_2() -> u64 {
    let lines = get_lines_from_file("../inputs/7.txt");

    let mut previous_row_beam_timelines: HashMap<usize, u64> = HashMap::new();

    for line in lines.iter() {
        let mut current_row_beam_timelines: HashMap<usize, u64> = HashMap::new();

        for (index, char) in line.chars().enumerate() {
            let mut beam_timeline_count = 0;
            if char == '.' {
                if previous_row_beam_timelines.contains_key(&index) {
                    beam_timeline_count += previous_row_beam_timelines.get(&index).unwrap()
                }

                if index > 0
                    && line.chars().nth(index - 1) == Some('^')
                    && previous_row_beam_timelines.contains_key(&(index - 1))
                {
                    beam_timeline_count += previous_row_beam_timelines.get(&(index - 1)).unwrap()
                }

                if line.chars().nth(index + 1) == Some('^')
                    && previous_row_beam_timelines.contains_key(&(index + 1))
                {
                    beam_timeline_count += previous_row_beam_timelines.get(&(index + 1)).unwrap()
                }
            } else if char == 'S' {
                beam_timeline_count = 1
            }

            if beam_timeline_count > 0 {
                current_row_beam_timelines.insert(index, beam_timeline_count);
            }
        }

        previous_row_beam_timelines = current_row_beam_timelines;
    }

    previous_row_beam_timelines.values().sum()
}
