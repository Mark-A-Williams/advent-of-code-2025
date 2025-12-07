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

pub fn part_2() -> i32 {
    return 0;
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
