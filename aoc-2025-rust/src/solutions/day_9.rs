use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u64 {
    let coords = parse_input();

    let mut biggest_area: u64 = 0;

    for first in &coords {
        for second in &coords {
            let area = (first.x.abs_diff(second.x) + 1) * (first.y.abs_diff(second.y) + 1);
            if area > biggest_area {
                biggest_area = area
            }
        }
    }

    biggest_area
}

pub fn part_2() -> u64 {
    0
}

fn parse_input() -> Vec<Coord> {
    let lines = get_lines_from_file("../inputs/9.txt");
    let mut result = vec![];
    for line in lines {
        let line_split: Vec<u64> = line.split(',').map(|o| o.parse::<u64>().unwrap()).collect();
        result.push(Coord {
            x: line_split[0],
            y: line_split[1],
        })
    }
    result
}

struct Coord {
    x: u64,
    y: u64,
}
