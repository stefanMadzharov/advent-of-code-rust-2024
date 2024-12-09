advent_of_code::solution!(6);

use Direction::*;

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

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
    direction: Direction,
    position: (usize, usize),
    unique_places_checked: u32,
    finished: bool,
    is_stuck: bool,
}

impl Guard {
    fn move_next_position(mut self, map: &mut Vec<Vec<char>>) -> Self {
        if let Some((row, col)) =
            self.direction
                .next_position(self.position, map.len(), map[0].len())
        {
            match map[row][col] {
                '#' => {
                    self.direction = self.direction.turn_right();
                    self
                }
                '.' => {
                    map[row][col] = 'X';
                    self.position = (row, col);
                    self.unique_places_checked += 1;
                    self
                }
                'X' | '>' | 'V' | '<' | '^' => {
                    map[row][col] = 'W';
                    self.position = (row, col);
                    self
                }
                'W' => {
                    map[row][col] = 'Q';
                    self.position = (row, col);
                    self
                }
                'Q' => {
                    self.is_stuck = true;
                    self
                }
                _ => unreachable!("There was an unexpected symbol on the map"),
            }
        } else {
            self.finished = true;
            self
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut guard = Guard {
        direction: Up,
        position: (0, 0),
        unique_places_checked: 1,
        finished: false,
        is_stuck: false,
    };
    for (i, row) in map.iter().enumerate() {
        if row.contains(&'^') {
            for (j, &char) in row.iter().enumerate() {
                if char == '^' {
                    guard.position = (i, j)
                }
            }
        }
    }
    while !guard.finished {
        guard = guard.move_next_position(&mut map)
    }
    Some(guard.unique_places_checked)
}

pub fn part_two(input: &str) -> Option<u32> {
    let init_map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut init_guard = Guard {
        direction: Up,
        position: (0, 0),
        unique_places_checked: 1,
        finished: false,
        is_stuck: false,
    };
    let mut unique_obstacles = 0;

    for (i, row) in init_map.iter().enumerate() {
        if row.contains(&'^') {
            for (j, &char) in row.iter().enumerate() {
                if char == '^' {
                    init_guard.position = (i, j)
                }
            }
        }
    }

    for (i, row) in init_map.clone().iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == '.'
                && (i != init_guard.position.0 || i != init_guard.position.0 - 1)
                && j != init_guard.position.1
            {
                let mut guard = init_guard.clone();
                let mut map = init_map.clone();

                map[i][j] = '#';
                while !guard.finished {
                    guard = guard.move_next_position(&mut map);
                    if guard.is_stuck {
                        unique_obstacles += 1;
                        break;
                    }
                }
            }
        }
    }
    Some(unique_obstacles)
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
