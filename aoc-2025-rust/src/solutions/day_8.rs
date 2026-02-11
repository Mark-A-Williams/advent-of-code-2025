use std::collections::HashSet;

use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> usize {
    let junction_boxes = parse_input();
    let connections = get_all_connections_by_distance(&junction_boxes);

    let mut circuits: Vec<HashSet<u16>> = junction_boxes
        .iter()
        .map(|j_box| HashSet::from([j_box.id]))
        .collect();

    for connection in connections.iter().take(1000) {
        let existing_first_box_circuit = circuits
            .iter()
            .enumerate()
            .find(|set| set.1.contains(&connection.id1))
            .unwrap();

        if existing_first_box_circuit.1.contains(&connection.id2) {
            continue;
        }

        let mut new_circuit = existing_first_box_circuit.1.clone();

        circuits.remove(existing_first_box_circuit.0);

        let existing_second_box_circuit = circuits
            .iter()
            .enumerate()
            .find(|set| set.1.contains(&connection.id2))
            .unwrap();

        new_circuit.extend(existing_second_box_circuit.1);

        circuits.remove(existing_second_box_circuit.0);

        circuits.push(new_circuit);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

pub fn part_2() -> u64 {
    let lines = get_lines_from_file("../inputs/8.example.txt");
    0
}

fn parse_input() -> Vec<JunctionBox> {
    let lines = get_lines_from_file("../inputs/8.txt");
    let mut result = vec![];
    for (index, line) in lines.iter().enumerate() {
        let line_split: Vec<i64> = line.split(',').map(|o| o.parse::<i64>().unwrap()).collect();
        result.push(JunctionBox {
            id: index as u16,
            x: line_split[0],
            y: line_split[1],
            z: line_split[2],
        })
    }
    result
}

fn get_all_connections_by_distance(
    junction_boxes: &Vec<JunctionBox>,
) -> Vec<JunctionBoxConnection> {
    let mut result = vec![];

    for first in junction_boxes {
        for second in junction_boxes {
            if first.id >= second.id {
                continue;
            }

            let distance = ((first.x - second.x).pow(2) as f32
                + (first.y - second.y).pow(2) as f32
                + (first.z - second.z).pow(2) as f32)
                .sqrt();

            result.push(JunctionBoxConnection {
                id1: first.id,
                id2: second.id,
                distance: distance,
            })
        }
    }

    result.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    result
}

struct JunctionBox {
    id: u16,
    x: i64,
    y: i64,
    z: i64,
}

struct JunctionBoxConnection {
    id1: u16,
    id2: u16,
    distance: f32,
}
