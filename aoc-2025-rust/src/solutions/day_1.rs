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

pub fn part_2() -> i32 {
    let lines = get_lines_from_file("../inputs/1.txt");

    let commands = lines
        .iter()
        .map(|s| s.replace("R", "").replace("L", "-").parse::<i32>().unwrap());

    let mut position: i32 = 50;
    let mut count_of_zero_positions = 0;

    for command in commands {
        // Positions will always be between 0 and 100 for simplicity
        let mut new_position = (position + command) % 100;

        if new_position < 0 {
            new_position += 100;
        }

        if new_position % 100 == 0
            || (position != 0 && command < 0 && new_position >= position)
            || (position != 0 && command > 0 && new_position <= position)
        {
            count_of_zero_positions += 1;
        }

        if command > 0 {
            count_of_zero_positions += command / 100;
        } else {
            count_of_zero_positions -= command / 100;
        }

        position = new_position;
    }

    return count_of_zero_positions;
}
