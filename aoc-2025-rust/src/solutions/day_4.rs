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
            if grid[y][x] == '@' && is_position_accessible(x, y, &grid) {
                accessible_rolls_count += 1;
            }

            x += 1;
        }

        y += 1;
    }

    accessible_rolls_count
}

pub fn part_2() -> u32 {
    let lines = get_lines_from_file("../inputs/4.txt");

    let width = lines.first().unwrap().len();
    let height = lines.len();

    let mut grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut removed_rolls_count_total = 0;

    loop {
        let removed_count = remove_all_accessible_rolls(&mut grid, width, height);

        if removed_count == 0 {
            break;
        }

        removed_rolls_count_total += removed_count;
    }

    removed_rolls_count_total
}

fn remove_all_accessible_rolls(grid: &mut Vec<Vec<char>>, width: usize, height: usize) -> u32 {
    let mut removed_rolls_count = 0;

    let mut y = 0;
    while y < height {
        let mut x = 0;

        while x < width {
            if grid[y][x] == '@' && is_position_accessible(x, y, &grid) {
                grid[y][x] = '.';
                removed_rolls_count += 1;
            }

            x += 1;
        }

        y += 1;
    }

    return removed_rolls_count;
}

fn is_position_accessible(x_centre: usize, y_centre: usize, grid: &Vec<Vec<char>>) -> bool {
    let mut adjacent_roll_count = 0;

    let mut y = if y_centre == 0 { 0 } else { y_centre - 1 };

    while y <= y_centre + 1 {
        let mut x = if x_centre == 0 { 0 } else { x_centre - 1 };

        while x <= x_centre + 1 {
            if (x != x_centre || y != y_centre) && is_position_roll(x, y, grid) {
                adjacent_roll_count += 1;
            }

            if adjacent_roll_count >= 4 {
                return false;
            }

            x += 1;
        }

        y += 1;
    }

    return true;
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
