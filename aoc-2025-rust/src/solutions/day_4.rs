use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u32 {
    let lines = get_lines_from_file("../inputs/4.txt");

    let width = lines.first().unwrap().len();
    let height = lines.len();

    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut accessible_rolls_count = 0;

    let mut y = 0;
    while y < height {
        let mut x = 0;

        while x < width {
            if grid[y][x] == '@' && !has_too_many_rolls_in_adjacent_positions(x, y, &grid) {
                accessible_rolls_count += 1;
            }

            x += 1;
        }

        y += 1;
    }

    accessible_rolls_count
}

pub fn part_2() -> u32 {
    0
}

fn has_too_many_rolls_in_adjacent_positions(
    x_centre: usize,
    y_centre: usize,
    grid: &Vec<Vec<char>>,
) -> bool {
    let mut adjacent_roll_count = 0;

    let mut y = if y_centre == 0 { 0 } else { y_centre - 1 };

    while y <= y_centre + 1 {
        let mut x = if x_centre == 0 { 0 } else { x_centre - 1 };

        while x <= x_centre + 1 {
            if (x != x_centre || y != y_centre) && is_position_roll(x, y, grid) {
                adjacent_roll_count += 1;
            }

            if adjacent_roll_count >= 4 {
                return true;
            }

            x += 1;
        }

        y += 1;
    }

    return false;
}

fn is_position_roll(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    match grid.get(y) {
        None => false,
        Some(row) => match row.get(x) {
            Some('@') => true,
            _ => false,
        },
    }
}
