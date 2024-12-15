advent_of_code::solution!(15);

use Direction::*;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_position(
        &self,
        position: (usize, usize),
        max_rows: usize,
        max_columns: usize,
    ) -> Option<(usize, usize)> {
        match *self {
            Up => Some((position.0.checked_sub(1)?, position.1)),
            Right => {
                if position.1 + 1 < max_columns {
                    Some((position.0, position.1 + 1))
                } else {
                    None
                }
            }
            Down => {
                if position.0 + 1 < max_rows {
                    Some((position.0 + 1, position.1))
                } else {
                    None
                }
            }
            Left => Some((position.0, position.1.checked_sub(1)?)),
        }
    }
}

#[derive(Clone)]
struct Guard {
    position: (usize, usize),
}

impl Guard {
    fn init(map: &Vec<Vec<char>>) -> Option<Self> {
        for (i, row) in map.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == '@' {
                    return Some(Guard { position: (i, j) });
                }
            }
        }
        None
    }

    fn do_next_move(&mut self, map: &mut Vec<Vec<char>>, direction: Direction) {
        if let Some((row, col)) = direction.next_position(self.position, map.len(), map[0].len()) {
            match map[row][col] {
                '#' => {
                    // println!("Pushing against the wall!");
                }
                '.' => {
                    map[row][col] = '@';
                    map[self.position.0][self.position.1] = '.';
                    self.position = (row, col);
                }
                'O' => {
                    let (mut ghost_i, mut ghost_j) = (row, col);
                    while let Some((i, j)) =
                        direction.next_position((ghost_i, ghost_j), map.len(), map[0].len())
                    {
                        (ghost_i, ghost_j) = (i, j);
                        match map[i][j] {
                            '#' => {
                                // println!("Pushing against the wall through boxes!");
                                break;
                            }
                            '.' => {
                                map[i][j] = 'O';
                                map[row][col] = '@';
                                map[self.position.0][self.position.1] = '.';
                                self.position = (row, col);
                                break;
                            }
                            'O' => {}
                            _ => {
                                eprintln!("Found {} on the map during ghost pathing!", map[i][j]);
                                unreachable!("There was an unexpected symbol on the map")
                            }
                        }
                    }
                }
                _ => {
                    eprintln!("Found {} on the map!", map[row][col]);
                    unreachable!("There was an unexpected symbol on the map")
                }
            }
        } else {
            panic!("Trying to get out of bounds!")
        }
    }
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

fn calculate_gps_coordinates(map: &Vec<Vec<char>>) -> u32 {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, char)| **char == 'O')
                .map(|(j, _)| (100 * i + j) as u32)
                .sum::<u32>()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .filter(|line| line.contains('#'))
        .map(|line| line.chars().collect())
        .collect();
    let directions: Vec<Direction> = input
        .lines()
        .filter(|line| line.contains(['v', '<', '>', '^']))
        .flat_map(|line| line.chars())
        .map(|char| match char {
            'v' => Down,
            '<' => Left,
            '>' => Right,
            '^' => Up,
            _ => {
                eprintln!("Found {char} in directions!");
                unreachable!("Directions parsed bad")
            }
        })
        .collect();
    // _print_map(&map);
    let mut guard = Guard::init(&map).unwrap();
    for direction in directions {
        // println!("{direction:?}");
        guard.do_next_move(&mut map, direction);
    }
    // _print_map(&map);
    Some(calculate_gps_coordinates(&map))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_big() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
