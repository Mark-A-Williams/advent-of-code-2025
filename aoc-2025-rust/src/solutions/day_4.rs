use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u32 {
    let lines = get_lines_from_file("../inputs/4.example.txt");

    let width = lines.first().unwrap().len();
    let height = lines.len();

    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut accessible_rolls_count = 0;

    let mut x = 0;
    while x < width {
        let mut y = 0;

        while y < height {
            if grid[y][x] == '@' && count_rolls_in_adjacent_positions(x, y, &grid) < 4 {
                accessible_rolls_count += 1;
            }

            y += 1;
        }

        x += 1;
    }

    accessible_rolls_count
}

pub fn part_2() -> u32 {
    0
}

fn count_rolls_in_adjacent_positions(x: usize, y: usize, grid: &Vec<Vec<char>>) -> u32 {
    0
}
