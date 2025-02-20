advent_of_code::solution!(12);

use itertools::{Itertools, MinMaxResult::*};
use Direction::*;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn get(
        &self,
        map: &Vec<Vec<char>>,
        coordinates: (usize, usize),
    ) -> Option<(char, (usize, usize))> {
        let (i, j) = coordinates;
        match *self {
            Up => {
                let new_coordinates = (i.checked_sub(1)?, j);
                let (new_i, new_j) = new_coordinates;
                Some((map[new_i][new_j], new_coordinates))
            }
            Down => {
                let new_coordinates = (i + 1, j);
                let (new_i, new_j) = new_coordinates;
                Some((map.get(new_i)?[new_j], new_coordinates))
            }
            Left => {
                let new_coordinates = (i, j.checked_sub(1)?);
                let (new_i, new_j) = new_coordinates;
                Some((map[new_i][new_j], new_coordinates))
            }
            Right => {
                let new_coordinates = (i, j + 1);
                let (new_i, new_j) = new_coordinates;
                Some((*map[new_i].get(new_j)?, new_coordinates))
            }
        }
    }
}
const DIRECTIONS: [Direction; 4] = [Up, Down, Right, Left];

#[derive(Debug)]
struct Region {
    symbol: char,
    area: u32,
    perimeter: u32,
    cells: Vec<(usize, usize)>,
}

impl Region {
    fn new(symbol: char) -> Self {
        Self {
            symbol,
            area: 0,
            perimeter: 0,
            cells: vec![],
        }
    }

    fn process_region(
        &mut self,
        map: &Vec<Vec<char>>,
        map_processed: &mut Vec<Vec<bool>>,
        coordinates: (usize, usize),
    ) {
        let (i, j) = coordinates;
        map_processed[i][j] = true;
        self.area += 1;
        self.cells.push(coordinates);
        for direction in DIRECTIONS {
            if let Some((symbol, new_coordinates)) = direction.get(&map, coordinates) {
                if symbol == self.symbol {
                    if !map_processed[new_coordinates.0][new_coordinates.1] {
                        self.process_region(map, map_processed, new_coordinates);
                    }
                } else {
                    self.perimeter += 1;
                }
            } else {
                self.perimeter += 1;
            }
        }
    }

    #[inline]
    fn is_contained(
        &self,
        coordinates: (i32, i32),
        top_left_corner: (i32, i32),
        swapped: bool,
    ) -> bool {
        if coordinates.0 <= top_left_corner.0 {
            return false;
        }
        let actual_coordinates = if swapped {
            (coordinates.1 as usize, coordinates.0 as usize)
        } else {
            (coordinates.0 as usize, coordinates.1 as usize)
        };
        self.cells.contains(&actual_coordinates)
    }

    fn get_horizontal(
        &mut self,
        min_max_row_index: (i32, i32),
        min_max_column_index: (i32, i32),
        swapped: bool,
    ) -> u32 {
        let (first_row_index, last_row_index) = min_max_row_index;
        let (first_column_index, last_column_index) = min_max_column_index;

        let top_left_corner: (i32, i32) = (first_row_index - 1, first_column_index - 1);
        let mut edges = 0;
        let mut down_edge_started = false;
        for i in first_row_index - 1..=last_row_index {
            for j in first_column_index..=last_column_index {
                if self.is_contained((i, j), top_left_corner, swapped) {
                    if self.is_contained((i + 1, j), top_left_corner, swapped) {
                        if down_edge_started {
                            edges += 1;
                            down_edge_started = false;
                        }
                    } else {
                        if down_edge_started {
                            if self.is_contained((i + 1, j - 1), top_left_corner, swapped)
                                && !self.is_contained((i, j - 1), top_left_corner, swapped)
                            {
                                edges += 1;
                            }
                        } else {
                            down_edge_started = true;
                        }
                    }
                } else {
                    if self.is_contained((i + 1, j), top_left_corner, swapped) {
                        if !down_edge_started {
                            down_edge_started = true;
                        } else {
                            if !self.is_contained((i + 1, j - 1), top_left_corner, swapped)
                                && self.is_contained((i, j - 1), top_left_corner, swapped)
                            {
                                edges += 1;
                            }
                        }
                    } else {
                        if down_edge_started {
                            edges += 1;
                            down_edge_started = false;
                        }
                    }
                }
            }
            if down_edge_started {
                edges += 1;
                down_edge_started = false;
            }
        }
        edges
    }

    fn get_vertical(
        &mut self,
        min_max_row_index: (i32, i32),
        min_max_column_index: (i32, i32),
    ) -> u32 {
        self.get_horizontal(min_max_column_index, min_max_row_index, true)
    }

    fn get_bulk_perimeter(&mut self) -> u32 {
        let min_max_rows = self.cells.iter().map(|(i, _)| *i as i32).unique().minmax();
        let min_max_columns = self.cells.iter().map(|(_, j)| *j as i32).unique().minmax();

        match min_max_rows {
            NoElements => unreachable!("Rows empty!"),
            OneElement(_) => 4,
            MinMax(first_row_index, last_row_index) => match min_max_columns {
                NoElements => unreachable!("Columns empty!"),
                OneElement(_) => 4,
                MinMax(first_column_index, last_column_index) => {
                    let horizontal_edges = self.get_horizontal(
                        (first_row_index, last_row_index),
                        (first_column_index, last_column_index),
                        false,
                    );
                    let vertical_edges = self.get_vertical(
                        (first_row_index, last_row_index),
                        (first_column_index, last_column_index),
                    );
                    horizontal_edges + vertical_edges
                }
            },
        }
    }
}

fn find_regions(map: &Vec<Vec<char>>) -> Vec<Region> {
    let mut regions = vec![];
    let mut map_processed: Vec<Vec<bool>> = map
        .iter()
        .map(|row| row.iter().map(|_| false).collect())
        .collect();
    for (i, row) in map.iter().enumerate() {
        for (j, symbol) in row.iter().enumerate() {
            if !map_processed[i][j] {
                let mut new_region = Region::new(*symbol);
                new_region.process_region(&map, &mut map_processed, (i, j));
                regions.push(new_region);
            }
        }
    }
    regions
}

fn _print_map(map: &Vec<Vec<char>>) {
    println!("{}", String::from("_").repeat(120));
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let regions = find_regions(&map);
    Some(
        regions
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let mut regions = find_regions(&map);
    Some(
        regions
            .iter_mut()
            .map(|region| {
                let bulk_parameter = region.get_bulk_perimeter();
                region.area * bulk_parameter
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }
}
