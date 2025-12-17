#![allow(dead_code)]

mod solutions;

use std::time::SystemTime;

use crate::solutions::day_5::{part_1, part_2};

fn main() {
    let time_1 = SystemTime::now();

    let part_1_result = part_1();
    let part_1_time = time_1.elapsed().unwrap();

    println!(
        "Part 1 result: {0} (duration {1}ms)",
        part_1_result,
        part_1_time.as_millis()
    );

    let time_2 = SystemTime::now();

    let part_2_result = part_2();
    let part_2_time = time_2.elapsed().unwrap();

    println!(
        "Part 2 result: {0} (duration {1}ms)",
        part_2_result,
        part_2_time.as_millis()
    );
}
