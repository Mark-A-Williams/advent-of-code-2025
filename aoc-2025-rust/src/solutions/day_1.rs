use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> i32 {
    let lines = get_lines_from_file("../inputs/1.txt");

    let commands = lines
        .iter()
        .map(|s| s.replace("R", "").replace("L", "-").parse::<i32>().unwrap());

    let mut position = 50;
    let mut count_of_zero_positions = 0;

    for command in commands {
        position += command;

        if position % 100 == 0 {
            count_of_zero_positions += 1;
        }
    }

    return count_of_zero_positions;
}
