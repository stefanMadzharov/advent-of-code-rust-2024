#![feature(let_chains)]
advent_of_code::solution!(12);
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

    fn go_left(&self, coordinates: (usize, usize)) -> Option<(usize, usize)> {
        let (i, j) = coordinates;
        match *self {
            Up => Some((i, j.checked_sub(1)?)),
            Down => Some((i, j + 1)),
            Left => Some((i + 1, j)),
            Right => Some((i.checked_sub(1)?, j)),
        }
    }

    fn go_forward(&self, coordinates: (usize, usize)) -> Option<(usize, usize)> {
        let (i, j) = coordinates;
        match *self {
            Up => Some((i.checked_sub(1)?, j)),
            Down => Some((i + 1, j)),
            Left => Some((i, j.checked_sub(1)?)),
            Right => Some((i, j + 1)),
        }
    }
}
const DIRECTIONS: [Direction; 4] = [Up, Down, Right, Left];

struct BulkCounter {
    direction: Direction,
    position: (usize, usize),
    finished: bool,
    started: bool,
    consecutive_turns: u8,
    edges: u32,
}

impl BulkCounter {
    fn new(position: (usize, usize)) -> Self {
        Self {
            direction: Up,
            position,
            finished: false,
            started: false,
            consecutive_turns: 0,
            edges: 0,
        }
    }

    fn move_next_position(&mut self, coordinates: &Vec<(usize, usize)>) {
        if self.consecutive_turns < 4 {
            if self.started && self.position == coordinates[0] {
                if self.direction == Left {
                    self.edges += 1;
                }
                self.finished = true
            } else {
                if let Some(future_position) = self.direction.go_left(self.position)
                    && coordinates.contains(&future_position)
                {
                    match self.direction {
                        Up => self.direction = Left,
                        Left => self.direction = Down,
                        Down => self.direction = Right,
                        Right => self.direction = Up,
                    };
                    self.edges += 1;
                    self.position = future_position;
                } else {
                    if let Some(future_position) = self.direction.go_forward(self.position)
                        && coordinates.contains(&future_position)
                    {
                        self.position = future_position;
                        self.started = true;
                        self.consecutive_turns = 0;
                    } else {
                        match self.direction {
                            Up => self.direction = Right,
                            Right => self.direction = Down,
                            Down => self.direction = Left,
                            Left => self.direction = Up,
                        };
                        self.consecutive_turns += 1;
                        self.edges += 1;
                    }
                }
            }
        } else {
            self.finished = true;
        }
    }
}

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

    fn get_bulk_perimeter(&mut self) -> u32 {
        self.cells.sort_by(|a, b| {
            if a.0.cmp(&b.0).is_eq() {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        let mut bulk_counter = BulkCounter::new(self.cells[0]);
        while !bulk_counter.finished {
            bulk_counter.move_next_position(&self.cells);
        }
        bulk_counter.edges
    }

    fn contains(&mut self, other: &mut Region) -> bool {
        todo!()
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
    let regions = find_regions(map);
    Some(
        regions
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    _print_map(&map);
    let mut regions = find_regions(map);
    for region in regions.iter_mut() {
        println!(
            "{region:?} with bulk perimeter: {}",
            region.get_bulk_perimeter()
        );
    }
    Some(
        regions
            .iter_mut()
            .map(|region| region.area * region.get_bulk_perimeter())
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
