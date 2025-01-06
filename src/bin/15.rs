advent_of_code::solution!(15);

use Direction::*;

#[derive(Clone, Debug, PartialEq, Eq)]
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

    fn can_push_wide(&mut self, map: &Vec<Vec<char>>, direction: &Direction) -> bool {
        if *direction == Right || *direction == Left {
            let (mut ghost_i, mut ghost_j) = self.position;
            while let Some((i, j)) =
                direction.next_position((ghost_i, ghost_j), map.len(), map[0].len())
            {
                (ghost_i, ghost_j) = (i, j);
                match map[i][j] {
                    '#' => {
                        return false;
                    }
                    '.' => {
                        return true;
                    }
                    '[' | ']' => {}
                    _ => {
                        eprintln!("Found {} on the map during ghost pathing!", map[i][j]);
                        unreachable!("There was an unexpected symbol on the map")
                    }
                }
            }
            return false; // unreachable
        } else {
            if let Some((i, j)) = direction.next_position(self.position, map.len(), map[0].len()) {
                match map[i][j] {
                    '#' => {
                        return false;
                    }
                    '.' => {
                        return true;
                    }
                    '[' => {
                        let mut right_side = self.clone();
                        right_side.position = (i, j + 1);

                        let mut left_side = self.clone();
                        left_side.position = (i, j);
                        return right_side.can_push_wide(map, direction)
                            && left_side.can_push_wide(map, direction);
                    }
                    ']' => {
                        let mut right_side = self.clone();
                        right_side.position = (i, j);

                        let mut left_side = self.clone();
                        left_side.position = (i, j - 1);
                        return right_side.can_push_wide(map, direction)
                            && left_side.can_push_wide(map, direction);
                    }
                    _ => {
                        eprintln!("Found {} on the map during ghost pathing!", map[i][j]);
                        unreachable!("There was an unexpected symbol on the map")
                    }
                }
            }
            return false;
        }
    }

    fn push_wide(&mut self, map: &mut Vec<Vec<char>>, direction: &Direction) {
        if *direction == Right || *direction == Left {
            let (mut ghost_i, mut ghost_j) = self.position;
            let mut last_char = '@';
            map[self.position.0][self.position.1] = '.';
            if let Some((i, j)) =
                direction.next_position((ghost_i, ghost_j), map.len(), map[0].len())
            {
                self.position = (i, j);
            }
            while let Some((i, j)) =
                direction.next_position((ghost_i, ghost_j), map.len(), map[0].len())
            {
                (ghost_i, ghost_j) = (i, j);
                match map[i][j] {
                    '.' => {
                        map[i][j] = last_char;
                        break;
                    }
                    '[' | ']' => {
                        let temp = map[i][j];
                        map[i][j] = last_char;
                        last_char = temp;
                    }
                    _ => {
                        eprintln!("Found {} on the map during pushing!", map[i][j]);
                        unreachable!("There was an unexpected symbol on the map")
                    }
                }
            }
        } else {
            if let Some((i, j)) = direction.next_position(self.position, map.len(), map[0].len()) {
                match map[i][j] {
                    '[' => {
                        let mut right_side = self.clone();
                        right_side.position = (i, j + 1);

                        let mut left_side = self.clone();
                        left_side.position = (i, j);
                        right_side.push_wide(map, direction);
                        left_side.push_wide(map, direction);
                        self.push_wide(map, direction);
                        self.position = (i, j);
                    }
                    ']' => {
                        let mut right_side = self.clone();
                        right_side.position = (i, j);

                        let mut left_side = self.clone();
                        left_side.position = (i, j - 1);

                        right_side.push_wide(map, direction);
                        left_side.push_wide(map, direction);
                        self.push_wide(map, direction);
                        self.position = (i, j);
                    }
                    '.' => {
                        map[i][j] = map[self.position.0][self.position.1];
                        map[self.position.0][self.position.1] = '.';
                    }
                    _ => {
                        eprintln!("Found {} on the map during pushing!", map[i][j]);
                        unreachable!("There was an unexpected symbol on the map")
                    }
                }
            }
        }
        // _print_map(map);
    }

    fn do_next_move_wide(&mut self, map: &mut Vec<Vec<char>>, direction: &Direction) {
        if let Some((row, col)) = direction.next_position(self.position, map.len(), map[0].len()) {
            match map[row][col] {
                '#' => {}
                '.' => {
                    map[row][col] = '@';
                    map[self.position.0][self.position.1] = '.';
                    self.position = (row, col);
                    return;
                }
                '[' | ']' => {
                    if let Some((i, j)) =
                        direction.next_position(self.position, map.len(), map[0].len())
                    {
                        match map[i][j] {
                            '#' => {
                                // println!("Pushing against the wall through boxes!");
                            }
                            '.' => {
                                map[row][col] = '@';
                                map[self.position.0][self.position.1] = '.';
                                self.position = (row, col);
                            }
                            '[' | ']' => {
                                if self.can_push_wide(map, direction) {
                                    self.push_wide(map, direction);
                                }
                            }
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
                .filter(|(_, char)| **char == 'O' || **char == '[')
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .filter(|line| line.contains('#'))
        .map(|line| {
            let mut new_line = vec![];
            for char in line.chars() {
                match char {
                    '#' => {
                        new_line.push('#');
                        new_line.push('#');
                    }
                    'O' => {
                        new_line.push('[');
                        new_line.push(']');
                    }
                    '.' => {
                        new_line.push('.');
                        new_line.push('.');
                    }
                    '@' => {
                        new_line.push('@');
                        new_line.push('.');
                    }
                    _ => panic!("Unknown char in map"),
                }
            }
            new_line
        })
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
        guard.do_next_move_wide(&mut map, &direction);
        // _print_map(&map);
    }
    Some(calculate_gps_coordinates(&map))
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
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_part_two_big() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
