advent_of_code::solution!(20);
use Direction::*;

type Position = (usize, usize);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_position<T: Copy>(&self, map: &Vec<Vec<T>>, position: Position) -> (T, Position) {
        match *self {
            Up => {
                let (i, j) = (position.0 - 1, position.1);
                (map[i][j], (i, j))
            }
            Right => {
                let (i, j) = (position.0, position.1 + 1);
                (map[i][j], (i, j))
            }
            Down => {
                let (i, j) = (position.0 + 1, position.1);
                (map[i][j], (i, j))
            }
            Left => {
                let (i, j) = (position.0, position.1 - 1);
                (map[i][j], (i, j))
            }
        }
    }

    fn _jump_position<T: Copy>(
        &self,
        map: &Vec<Vec<T>>,
        position: Position,
    ) -> Option<(T, Position)> {
        match *self {
            Up => {
                let (i, j) = (position.0.checked_sub(2)?, position.1);
                Some((map.get(i)?[position.1], (i, j)))
            }
            Right => {
                let (i, j) = (position.0, position.1 + 2);
                Some((*map[i].get(j)?, (i, j)))
            }
            Down => {
                let (i, j) = (position.0 + 2, position.1);
                Some((map.get(i)?[j], (i, j)))
            }
            Left => {
                let (i, j) = (position.0, position.1.checked_sub(2)?);
                Some((*map[i].get(position.1)?, (i, j)))
            }
        }
    }

    fn can_turn_left(&self, map: &Vec<Vec<char>>, position: Position) -> bool {
        let new_dir = self.turn_left();
        let (symbol, _position) = new_dir.next_position(map, position);
        symbol == '.'
    }

    fn turn_left(&self) -> Self {
        match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn can_turn_right(&self, map: &Vec<Vec<char>>, position: Position) -> bool {
        let new_dir = self.turn_right();
        // new_dir.next_position(map, position).0 == '.'
        let (symbol, _position) = new_dir.next_position(map, position);
        symbol == '.'
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

struct CheaterProgram {
    char_map: Vec<Vec<char>>,
    int_map: Vec<Vec<i32>>,
    direction: Direction,
    position: Position,
    path_len: u32,
}

impl CheaterProgram {
    fn init(map: &Vec<Vec<char>>) -> Self {
        let vec = vec![-1].repeat(map.len());
        let mut int_map = vec![vec.clone()];
        for _ in 0..map[0].len() {
            int_map.push(vec.clone());
        }
        CheaterProgram {
            char_map: map.clone(),
            int_map,
            direction: Up,
            position: (3, 1),
            path_len: 0,
        }
    }

    fn do_init_run(&mut self) {
        self.int_map[self.position.0][self.position.1] = 0;
        self.path_len += 1;
        loop {
            let (symbol, position) = self.direction.next_position(&self.char_map, self.position);
            match symbol {
                '.' => {
                    self.int_map[position.0][position.1] = self.path_len as i32;
                    self.path_len += 1;
                    self.position = position
                }
                '#' | 'E' => {
                    if self.direction.can_turn_left(&self.char_map, self.position) {
                        self.direction = self.direction.turn_left();
                    } else if self.direction.can_turn_right(&self.char_map, self.position) {
                        self.direction = self.direction.turn_right();
                    } else {
                        break;
                    }
                }
                _ => unreachable!("Found unknown char in the map"),
            }
        }
    }

    fn _print_map(&self) {
        for row in self.int_map.iter() {
            for c in row {
                if *c == -1 {
                    print!("  .  ");
                } else {
                    print!(" {:03?} ", c);
                }
            }
            println!("");
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut cheater = CheaterProgram::init(&map);
    cheater.do_init_run();
    cheater._print_map();

    // let (_path, score) = cheater.find_path_to_finish().unwrap();
    // Some(score)
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
