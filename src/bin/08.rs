advent_of_code::solution!(8);
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinates {
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
}

impl std::fmt::Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {})", self.row, self.column))
    }
}
impl Coordinates {
    fn new(row: i32, column: i32, max_row: usize, max_column: usize) -> Option<Self> {
        if row < 0 || column < 0 {
            return None;
        }
        let row = row as usize;
        let column = column as usize;
        if row < max_row && column < max_column {
            Some(Coordinates {
                row,
                column,
                max_column,
                max_row,
            })
        } else {
            None
        }
    }

    fn calculate_single_pair_antinodes(&self, other: &Coordinates) -> [Option<Self>; 2] {
        let row_diff = self.row as i32 - other.row as i32;
        let column_diff = self.column as i32 - other.column as i32;
        let first_antinode = Self::new(
            self.row as i32 + row_diff,
            self.column as i32 + column_diff,
            self.max_row,
            self.max_column,
        );
        let second_antinode = Self::new(
            other.row as i32 - row_diff,
            other.column as i32 - column_diff,
            self.max_row,
            self.max_column,
        );
        [first_antinode, second_antinode]
    }

    fn calculate_multi_pair_antinodes(&self, other: &Coordinates) -> Vec<Self> {
        let mut result = vec![];
        let row_diff = self.row as i32 - other.row as i32;
        let column_diff = self.column as i32 - other.column as i32;
        for i in 1.. {
            if let Some(next_antinode) = Self::new(
                self.row as i32 + i * row_diff,
                self.column as i32 + i * column_diff,
                self.max_row,
                self.max_column,
            ) {
                result.push(next_antinode)
            } else {
                break;
            }
        }
        for i in 0.. {
            if let Some(next_antinode) = Self::new(
                other.row as i32 - i * row_diff,
                other.column as i32 - i * column_diff,
                self.max_row,
                self.max_column,
            ) {
                result.push(next_antinode)
            } else {
                break;
            }
        }
        result
    }
}

fn find_antenna_coordinates(antenna_type: char, antenna_map: &Vec<Vec<char>>) -> Vec<Coordinates> {
    let mut antenna_type_coordinates = vec![];
    for (i, row) in antenna_map.iter().enumerate() {
        for (j, _) in row
            .iter()
            .enumerate()
            .filter(|(_, char)| **char == antenna_type)
        {
            antenna_type_coordinates.push(
                Coordinates::new(i as i32, j as i32, antenna_map.len(), antenna_map[0].len())
                    .unwrap(),
            )
        }
    }
    antenna_type_coordinates
}

fn find_antinodes(antenna_types: &HashSet<char>, antenna_map: &Vec<Vec<char>>) -> u32 {
    let mut unique_antinodes: HashSet<Coordinates> = HashSet::new();
    for antenna_type in antenna_types {
        let antena_type_coordinates = find_antenna_coordinates(*antenna_type, antenna_map);
        let count_coordinates = antena_type_coordinates.len();
        for i in 0..count_coordinates {
            for j in 0..count_coordinates {
                if i != j {
                    antena_type_coordinates[i]
                        .calculate_single_pair_antinodes(&antena_type_coordinates[j])
                        .into_iter()
                        .filter(|coordinate| coordinate.is_some())
                        .for_each(|coordinate| {
                            if coordinate.is_some() {
                                unique_antinodes.insert(coordinate.clone().unwrap());
                            }
                        })
                }
            }
        }
    }
    return unique_antinodes.len() as u32;
}

fn find_antinodes_multi(antenna_types: &HashSet<char>, antenna_map: &Vec<Vec<char>>) -> u32 {
    let mut unique_antinodes: HashSet<Coordinates> = HashSet::new();
    for antenna_type in antenna_types {
        let antena_type_coordinates = find_antenna_coordinates(*antenna_type, antenna_map);
        let count_coordinates = antena_type_coordinates.len();
        for i in 0..count_coordinates {
            for j in 0..count_coordinates {
                if i != j {
                    antena_type_coordinates[i]
                        .calculate_multi_pair_antinodes(&antena_type_coordinates[j])
                        .into_iter()
                        .for_each(|coordinate| {
                            unique_antinodes.insert(coordinate.clone());
                        })
                }
            }
        }
    }
    return unique_antinodes.len() as u32;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut antenna_types: HashSet<char> = HashSet::new();
    for char in input.chars().filter(|char| *char != '.' && *char != '\n') {
        antenna_types.insert(char);
    }
    let antenna_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Some(find_antinodes(&antenna_types, &antenna_map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antenna_types: HashSet<char> = HashSet::new();
    for char in input.chars().filter(|char| *char != '.' && *char != '\n') {
        antenna_types.insert(char);
    }
    let antenna_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Some(find_antinodes_multi(&antenna_types, &antenna_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
