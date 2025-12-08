use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u64 {
    let lines = get_lines_from_file("../inputs/3.txt");
    let mut result = 0;
    for line in lines {
        let increment = get_largest_n_digit_num_from_line(line, 2);
        result += increment;
    }

    return result;
}

pub fn part_2() -> u64 {
    let lines = get_lines_from_file("../inputs/3.txt");
    let mut result = 0;
    for line in lines {
        let increment = get_largest_n_digit_num_from_line(line, 12);
        result += increment;
    }

    return result;
}

fn get_largest_n_digit_num_from_line(line: String, n: i32) -> u64 {
    let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut previous_digit_index: i32 = -1;
    let mut n_remaining = n - 1;
    let mut result: u64 = 0;

    while n_remaining >= 0 {
        let next_digit = get_next_digit(&digits, n_remaining, previous_digit_index);
        previous_digit_index = next_digit.1 as i32;

        result += (next_digit.0 as i64 * 10_i64.pow(n_remaining as u32)) as u64;

        n_remaining -= 1;
    }

    result
}

// Returns (digit, index)
fn get_next_digit(digits: &Vec<u32>, n_remaining: i32, previous_digit_index: i32) -> (u32, usize) {
    let mut current_index = 0;
    let mut return_digit = 0;

    for (count, digit) in digits.iter().enumerate() {
        if count as i32 > previous_digit_index
            && count < digits.len() - n_remaining as usize
            && digit > &return_digit
        {
            return_digit = *digit;
            current_index = count;
        }
    }

    (return_digit, current_index)
}
