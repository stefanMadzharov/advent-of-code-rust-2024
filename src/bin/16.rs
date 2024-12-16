advent_of_code::solution!(16);
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
    ) -> (usize, usize) {
        match *self {
            Up => (position.0 - 1, position.1),
            Right => (position.0, position.1 + 1),
            Down => (position.0 + 1, position.1),
            Left => (position.0, position.1 - 1),
        }
    }

    fn turn_left(&self) -> Self {
        match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn turn_right(&self) -> Self {
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Clone)]
struct Reindeer {
    position: (usize, usize),
    direction: Direction,
    finished: bool,
}

impl Reindeer {
    fn init(map: &Vec<Vec<char>>) -> Option<Self> {
        for (i, row) in map.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == 'S' {
                    return Some(Self {
                        position: (i, j),
                        direction: Right,
                        finished: false,
                    });
                }
            }
        }
        None
    }

    fn can_turn_left(&self, map: &mut Vec<Vec<char>>) -> bool {
        let (i, j) =
            self.direction
                .turn_left()
                .next_position(self.position, map.len(), map[0].len());
        map[i][j] == '.'
    }

    fn can_turn_right(&self, map: &mut Vec<Vec<char>>) -> bool {
        let (i, j) =
            self.direction
                .turn_right()
                .next_position(self.position, map.len(), map[0].len());
        map[i][j] == '.'
    }

    fn do_next_move(&mut self, map: &mut Vec<Vec<char>>, direction: Direction) {
        let (row, col) = direction.next_position(self.position, map.len(), map[0].len());
        match map[row][col] {
            '#' => {
                // println!("Pushing against the wall!");
            }
            '.' => {
                self.position = (row, col);
                todo!();
            }
            'E' | 'S' => self.finished = true,
            _ => {
                eprintln!("Found {} on the map!", map[row][col]);
                unreachable!("There was an unexpected symbol on the map")
            }
        }
    }

    fn find_path_to_finish() {
        todo!()
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

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    _print_map(&map);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_big() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
