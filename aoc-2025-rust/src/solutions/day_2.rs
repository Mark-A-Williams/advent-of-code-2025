use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> i64 {
    let lines = get_lines_from_file("../inputs/2.txt");
    let ranges = parse_all_id_ranges(&lines.first().unwrap());

    for range in &ranges {
        println!("Range {0}", range.max - range.min)
    }

    return ranges
        .iter()
        .map(|o| o.get_sum_of_invalid_ids_in_range())
        .sum();

    // return 0;
}

pub fn part_2() -> i32 {
    0
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
}
