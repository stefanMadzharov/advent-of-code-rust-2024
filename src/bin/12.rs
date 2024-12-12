advent_of_code::solution!(12);
use Direction::*;

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
}

impl Region {
    fn new(symbol: char) -> Self {
        Self {
            symbol,
            area: 0,
            perimeter: 0,
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
}

fn find_regions(map: Vec<Vec<char>>) -> Vec<Region> {
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
    let regions = find_regions(map);
    Some(
        regions
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
