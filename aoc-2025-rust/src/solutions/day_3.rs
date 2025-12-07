use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u32 {
    let lines = get_lines_from_file("../inputs/3.txt");
    let mut result = 0;
    for line in lines {
        let increment = get_largest_two_digit_num_from_line(line);
        result += increment;
    }

    return result;
}

pub fn part_2() -> u32 {
    let lines = get_lines_from_file("../inputs/3.txt");
    let mut result = 0;
    for line in lines {
        let increment = get_largest_n_digit_num_from_line(line, 12);
        result += increment;
    }

    return result;
}

fn get_largest_two_digit_num_from_line(line: String) -> u32 {
    let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut first_digit: u32 = 0;
    let mut first_digit_index: usize = 0;

    for (count, digit) in digits.iter().enumerate() {
        if count < digits.len() - 1 && digit > &first_digit {
            first_digit = *digit;
            first_digit_index = count;
        }
    }

    let second_digit = digits.iter().skip(first_digit_index + 1).max().unwrap();

    return 10 * first_digit + second_digit;
}

fn get_largest_n_digit_num_from_line(line: String, n: i32) -> u32 {
    // For i from 12 to 1 (counting down probably easier)
    // Digit to choose is the largest one that is more than i from the end, and after the previously chosen one
    // If there are multiple of the same digit meeting those criteria, it is the first one
    let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut result_digits: Vec<u32> = vec![];
    let mut previous_digit_index: usize = 0;
    let mut n_remaining = 12;

    while (n_remaining > 0) {
        let next_digit = get_next_digit(&digits, n_remaining, &mut previous_digit_index);
        result_digits.push(next_digit);
        n_remaining -= 1;
    }

    // TODO turn digits into big number

    0
}

fn get_next_digit(digits: &Vec<u32>, n_remaining: i32, previous_digit_index: &mut usize) -> u32 {
    0
}
