use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> u64 {
    parse_input().iter().map(|p| p.get_result()).sum()
}

pub fn part_2() -> u64 {
    0
}

fn parse_input() -> Vec<Problem> {
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
