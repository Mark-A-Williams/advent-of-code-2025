use std::collections::{HashMap, HashSet};

use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() -> usize {
    let vertices = get_vertices();

    let mut biggest_area: usize = 0;

    for first in &vertices {
        for second in &vertices {
            let area = (first.x.abs_diff(second.x) + 1) * (first.y.abs_diff(second.y) + 1);
            if area > biggest_area {
                biggest_area = area
            }
        }
    }

    biggest_area
}

pub fn part_2() -> usize {
    let vertices = get_vertices();
    let mut edges: Vec<Edge> = vec![];

    for (i, vertex) in vertices.iter().enumerate() {
        let next_vertex: Coord = if i >= vertices.len() - 1 {
            vertices[0].clone()
        } else {
            vertices[i + 1].clone()
        };

        edges.push(Edge {
            start: vertex.clone(),
            end: next_vertex,
        });
    }

    let mut squashed_grid = create_squashed_grid(&vertices, &edges);

    squashed_grid.identify_all_empty_cells();

    // squashed_grid.print();

    let mut biggest_area: usize = 0;
    for first in &vertices {
        for second in &vertices {
            let first_value = squashed_grid.get_cell_value_at_original_coord(first.x, first.y);
            let second_value = squashed_grid.get_cell_value_at_original_coord(second.x, second.y);
            if !matches!(first_value.cell_type, CellType::Vertex)
                || !matches!(second_value.cell_type, CellType::Vertex)
            {
                continue;
            }

            let first_original_coord = first_value.original_coord;
            let second_original_coord = second_value.original_coord;

            let actual_area = (first_original_coord.x.abs_diff(second_original_coord.x) + 1)
                * (first_original_coord.y.abs_diff(second_original_coord.y) + 1);

            if actual_area < biggest_area {
                continue;
            }

            let is_valid = squashed_grid.check_validity_of_rectangle_with_corners(first, second);

            if is_valid {
                biggest_area = actual_area
            }
        }
    }

    biggest_area
}

fn get_vertices() -> Vec<Coord> {
    let lines = get_lines_from_file("../inputs/9.txt");
    let mut result = vec![];
    for line in lines {
        let line_split: Vec<usize> = line
            .split(',')
            .map(|o| o.parse::<usize>().unwrap())
            .collect();
        result.push(Coord {
            x: line_split[0],
            y: line_split[1],
        })
    }
    result
}

fn create_squashed_grid(vertices: &Vec<Coord>, edges: &Vec<Edge>) -> SquashedGrid {
    let mut non_redundant_x_coords: HashSet<usize> = HashSet::new();
    let mut non_redundant_y_coords: HashSet<usize> = HashSet::new();

    for vertex in vertices {
        non_redundant_x_coords.insert(vertex.x);
        non_redundant_y_coords.insert(vertex.y);

        non_redundant_x_coords.insert(vertex.x + 1);
        non_redundant_y_coords.insert(vertex.y + 1);
    }

    let mut non_redundant_x_coords: Vec<usize> = non_redundant_x_coords.into_iter().collect();
    non_redundant_x_coords.sort();
    let mut non_redundant_y_coords: Vec<usize> = non_redundant_y_coords.into_iter().collect();
    non_redundant_y_coords.sort();

    let mut squashed_grid_raw: Vec<Vec<SquashedGridCell>> = vec![];
    let mut original_coord_lookup: HashMap<Coord, Coord> = HashMap::new();

    for (i_y, y) in non_redundant_y_coords.iter().enumerate() {
        let mut row: Vec<SquashedGridCell> = vec![];
        for (i_x, x) in non_redundant_x_coords.iter().enumerate() {
            let cell_type: CellType;
            if vertices.iter().any(|v| v.x == *x && v.y == *y) {
                cell_type = CellType::Vertex
            } else if edges.iter().any(|e| e.contains_coord(*x, *y)) {
                cell_type = CellType::Edge
            } else if i_x == 0
                || i_x == non_redundant_x_coords.len() - 1
                || i_y == 0
                || i_y == non_redundant_y_coords.len() - 1
            {
                cell_type = CellType::Empty
            } else {
                cell_type = CellType::Unknown
            }

            let original_coord = Coord { x: *x, y: *y };

            row.push(SquashedGridCell {
                original_coord: original_coord,
                cell_type: cell_type,
            });

            original_coord_lookup.insert(original_coord, Coord { x: i_x, y: i_y });
        }

        squashed_grid_raw.push(row);
    }

    SquashedGrid {
        grid: squashed_grid_raw,
        original_coord_lookup,
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

struct Edge {
    start: Coord,
    end: Coord,
}

impl Edge {
    fn contains_coord(&self, x: usize, y: usize) -> bool {
        if x == self.start.x
            && x == self.end.x
            && y > self.start.y.min(self.end.y)
            && y < self.start.y.max(self.end.y)
        {
            return true;
        }

        if y == self.start.y
            && y == self.end.y
            && x > self.start.x.min(self.end.x)
            && x < self.start.x.max(self.end.x)
        {
            return true;
        }

        return false;
    }
}

struct SquashedGrid {
    grid: Vec<Vec<SquashedGridCell>>,
    // Keys are original coords, values are squashed coords
    original_coord_lookup: HashMap<Coord, Coord>,
}

impl SquashedGrid {
    fn check_validity_of_rectangle_with_corners(&self, first: &Coord, second: &Coord) -> bool {
        let squashed_first = self.original_coord_lookup[first];
        let squashed_second = self.original_coord_lookup[second];

        let smallest_x = squashed_first.x.min(squashed_second.x);
        let largest_x = squashed_first.x.max(squashed_second.x);
        let smallest_y = squashed_first.y.min(squashed_second.y);
        let largest_y = squashed_first.y.max(squashed_second.y);

        for y in smallest_y..largest_y {
            let left_cell_type = self.grid[y][smallest_x].cell_type;
            let right_cell_type = self.grid[y][largest_x].cell_type;

            if matches!(left_cell_type, CellType::Empty)
                || matches!(right_cell_type, CellType::Empty)
            {
                return false;
            }
        }

        for x in smallest_x..largest_x {
            let top_cell_type = self.grid[smallest_y][x].cell_type;
            let bottom_cell_type = self.grid[largest_y][x].cell_type;

            if matches!(top_cell_type, CellType::Empty)
                || matches!(bottom_cell_type, CellType::Empty)
            {
                return false;
            }
        }
        true
    }

    fn get_cell_value_at_original_coord(
        &self,
        original_x: usize,
        original_y: usize,
    ) -> SquashedGridCell {
        let original_coord = Coord {
            x: original_x,
            y: original_y,
        };
        let squashed_coord = self.original_coord_lookup[&original_coord];
        self.grid[squashed_coord.y][squashed_coord.x]
    }

    fn identify_all_empty_cells(&mut self) {
        let mut found_more_empty_tiles: bool = false;

        let width = self.grid[0].len();
        let height = self.grid.len();
        loop {
            for y in 0..height {
                for x in 0..width {
                    if !matches!(&self.grid[y][x].cell_type, CellType::Unknown) {
                        continue;
                    }

                    if matches!(&self.grid[y][x - 1].cell_type, CellType::Empty)
                        || matches!(&self.grid[y][x + 1].cell_type, CellType::Empty)
                        || matches!(&self.grid[y - 1][x].cell_type, CellType::Empty)
                        || matches!(&self.grid[y + 1][x].cell_type, CellType::Empty)
                    {
                        self.grid[y][x].cell_type = CellType::Empty;
                        found_more_empty_tiles = true;
                    }
                }
            }

            if !found_more_empty_tiles {
                break;
            }

            found_more_empty_tiles = false;
        }
    }

    fn print(&self) {
        for y in &self.grid {
            for x in y {
                let cell_type = x.cell_type;
                match cell_type {
                    CellType::Vertex => print!("O"),
                    CellType::Edge => print!("X"),
                    CellType::Empty => print!("-"),
                    CellType::Unknown => print!("*"),
                }
            }
            println!();
        }
    }
}

#[derive(Copy, Clone)]
struct SquashedGridCell {
    original_coord: Coord,
    cell_type: CellType,
}

#[derive(Copy, Clone)]
enum CellType {
    Vertex,
    Edge,
    Empty,
    Unknown,
}
