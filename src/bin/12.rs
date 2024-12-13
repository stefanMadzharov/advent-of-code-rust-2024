#![feature(let_chains)]
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

    fn get_horizontal(
        &mut self,
        map: &Vec<Vec<char>>,
        min_max_row_index: (usize, usize),
        min_max_column_index: (usize, usize),
    ) -> u32 {
        let (first_row_index, last_row_index) = min_max_row_index;
        let (first_column_index, last_column_index) = min_max_column_index;

        let mut zoomed_map: Vec<Vec<char>> = map[first_row_index..=last_row_index]
            .iter()
            .enumerate()
            .map(|(_, row)| row[first_column_index..=last_column_index].to_vec())
            .collect();
        zoomed_map.insert(
            0,
            map[first_row_index][first_column_index..=last_column_index]
                .iter()
                .map(|symbol| if *symbol == self.symbol { '.' } else { *symbol })
                .collect(),
        );
        zoomed_map.push(
            map[last_row_index][first_column_index..=last_column_index]
                .iter()
                .map(|symbol| if *symbol == self.symbol { '.' } else { *symbol })
                .collect(),
        );

        println!("Extended zoomed map:");
        _print_map(&zoomed_map);

        let edge_map: Vec<Vec<char>> = zoomed_map
            .iter()
            .tuple_windows()
            .map(|(i1, i2)| {
                i1.iter()
                    .zip(i2.iter())
                    .map(|(char1, char2)| if char1 == char2 { '.' } else { '_' })
                    .collect()
            })
            .collect();

        println!("Edge map:");
        _print_map(&edge_map);

        edge_map
            .iter()
            .map(|row| {
                let count = row
                    .iter()
                    .join("")
                    .replace("_.", "X.")
                    .replace("._", ".X")
                    .matches("X")
                    .count() as u32;
                if row.contains(&'_') && count == 0 {
                    1
                } else {
                    count
                }
            })
            .sum::<u32>()
    }

    fn get_vertical(
        &mut self,
        map: &Vec<Vec<char>>,
        min_max_row_index: (usize, usize),
        min_max_column_index: (usize, usize),
    ) -> u32 {
        self.get_horizontal(
            &Region::swap_map(map),
            min_max_column_index,
            min_max_row_index,
        )
    }

    fn swap_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut swapped_map: Vec<Vec<char>> = Vec::new();
        for _ in 0..map.len() {
            swapped_map.push(vec![]);
        }
        for row in map.iter() {
            for (j, char) in row.iter().enumerate() {
                swapped_map[j].push(*char);
            }
        }
        println!("Swapped map");
        _print_map(&swapped_map);
        swapped_map
    }

    fn get_bulk_perimeter(&mut self, map: &Vec<Vec<char>>) -> u32 {
        let min_max_rows = self.cells.iter().map(|(i, _)| *i).unique().minmax();
        let min_max_columns = self.cells.iter().map(|(_, j)| *j).unique().minmax();

        match min_max_rows {
            NoElements => unreachable!("Rows empty!"),
            OneElement(_) => 4,
            MinMax(first_row_index, last_row_index) => match min_max_columns {
                NoElements => unreachable!("Columns empty!"),
                OneElement(_) => 4,
                MinMax(first_column_index, last_column_index) => {
                    self.get_horizontal(
                        map,
                        (first_row_index, last_row_index),
                        (first_column_index, last_column_index),
                    ) + self.get_vertical(
                        map,
                        (first_row_index, last_row_index),
                        (first_column_index, last_column_index),
                    )
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
    println!("Normal map:");
    _print_map(&map);
    Some(
        regions
            .iter_mut()
            .map(|region| {
                let bulk_parameter = region.get_bulk_perimeter(&map);
                println!("{region:?} with bulk perimeter: {}\n", bulk_parameter);
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
