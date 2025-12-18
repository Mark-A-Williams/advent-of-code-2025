use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u64 {
    parse_input_normal_reading()
        .iter()
        .map(|p| p.get_result())
        .sum()
}

pub fn part_2() -> u64 {
    parse_input_columnwise()
        .iter()
        .map(|p| p.get_result())
        .sum()
}

fn parse_input_columnwise() -> Vec<Problem> {
    let lines = get_lines_from_file("../inputs/6.txt");

    let line_length = lines.first().unwrap().len();

    let mut input_values_buffer: Vec<u64> = vec![];
    let mut current_command: Option<char> = None;
    let mut current_column_index = 0;
    let mut results: Vec<Problem> = vec![];

    while current_column_index < line_length {
        let chars_from_all_lines = lines
            .iter()
            .map(|l| l.chars().nth(current_column_index).unwrap());

        if chars_from_all_lines
            .clone()
            .all(|c| c.is_ascii_whitespace())
        {
            results.push(Problem {
                inputs: input_values_buffer.clone(),
                command: current_command.unwrap(),
            });

            input_values_buffer.clear();
            current_command = None;

            current_column_index += 1;
            continue;
        }

        let numeric_string: String = chars_from_all_lines
            .clone()
            .filter(|c| c.is_digit(10))
            .collect();

        input_values_buffer.push(numeric_string.parse::<u64>().unwrap());

        let lowest_char = chars_from_all_lines.last().unwrap();

        if lowest_char.is_ascii_punctuation() {
            current_command = Some(lowest_char)
        }

        current_column_index += 1
    }

    results
}

fn parse_input_normal_reading() -> Vec<Problem> {
    let lines = get_lines_from_file("../inputs/6.txt");

    let mut result: Vec<Problem> = vec![];

    let commands_line = lines.last().unwrap();
    let commands = commands_line.split_ascii_whitespace();

    for command in commands {
        result.push(Problem {
            inputs: vec![],
            command: command.chars().nth(0).unwrap(),
        })
    }

    for line in lines.iter().take(lines.len() - 1) {
        let line_bits = line
            .split_ascii_whitespace()
            .map(|o| o.parse::<u64>())
            .enumerate();

        for (i, input_value) in line_bits {
            result[i].inputs.push(input_value.unwrap());
        }
    }

    result
}

struct Problem {
    inputs: Vec<u64>,
    command: char,
}

impl Problem {
    fn get_result(&self) -> u64 {
        let mut result = match self.command {
            '+' => 0,
            '*' => 1,
            _ => panic!("Unexpected command {0}", self.command),
        };

        for input in self.inputs.iter() {
            if self.command == '+' {
                result += input;
            } else {
                result *= input;
            }
        }

        result
    }
}
