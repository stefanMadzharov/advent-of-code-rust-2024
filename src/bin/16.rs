#![feature(let_chains)]
advent_of_code::solution!(16);
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use Direction::*;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
        }
    }
}

impl Direction {
    fn next_position(&self, position: (usize, usize)) -> (usize, usize) {
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

    fn turn_around(&self) -> Self {
        match *self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

#[derive(Clone)]
struct Reindeer {
    map: Vec<Vec<char>>,
    position: (usize, usize),
    direction: Direction,
    current_path: Vec<Direction>,
    current_min_path: (Vec<Direction>, u32),
    crossroads_to_check: Vec<((usize, usize), Direction, Vec<Direction>)>,
    crossroads_min_paths: HashMap<(usize, usize), HashMap<Direction, Vec<Direction>>>,
}

impl std::fmt::Debug for Reindeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "pos: {:?}, dir: {:?},\ncrossroads_to_check: {:?},\ncrossroads_min_paths:  {:?}\ncurrent_path: {:?}\n",
            self.position, self.direction, self.crossroads_to_check, self.crossroads_min_paths, self.current_path,
        ))
    }
}

impl Reindeer {
    fn init(map: &Vec<Vec<char>>) -> Option<Self> {
        for (i, row) in map.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == 'S' {
                    return Some(Self {
                        map: map.clone(),
                        position: (i, j),
                        direction: Right,
                        current_path: vec![Right],
                        current_min_path: (vec![], u32::MAX),
                        crossroads_to_check: vec![],
                        crossroads_min_paths: HashMap::new(),
                    });
                }
            }
        }
        None
    }

    fn can_turn_left(&self) -> bool {
        let (i, j) = self.direction.turn_left().next_position(self.position);
        self.map[i][j] == '.' || self.map[i][j] == 'E'
    }

    fn can_turn_right(&self) -> bool {
        let (i, j) = self.direction.turn_right().next_position(self.position);
        self.map[i][j] == '.' || self.map[i][j] == 'E'
    }

    fn can_continue_forward(&self) -> bool {
        let (i, j) = self.direction.next_position(self.position);
        self.map[i][j] == '.' || self.map[i][j] == 'E'
    }

    fn investigate_next_square(&mut self) -> bool {
        match self.map[self.position.0][self.position.1] {
            '#' => false,
            '.' => {
                let can_turn_right = self.can_turn_right();
                let can_turn_left = self.can_turn_left();

                if self.can_continue_forward() {
                    let updated_forward = if can_turn_left || can_turn_right {
                        self.update_min_crossroads(
                            self.position,
                            self.direction.clone().turn_around(),
                        );
                        self.update_min_crossroads(self.position, self.direction.clone())
                    } else {
                        true
                    };
                    let mut updated_left = false;
                    let mut updated_right = false;
                    if can_turn_right {
                        updated_right = self.update_min_crossroads(
                            self.position,
                            self.direction.clone().turn_right(),
                        );
                        if updated_right {
                            self.crossroads_to_check.push((
                                self.direction.turn_right().next_position(self.position),
                                self.direction.turn_right(),
                                {
                                    let mut path = self.current_path.clone();
                                    path.push(self.direction.turn_right());
                                    path
                                },
                            ));
                        }
                    }
                    if can_turn_left {
                        updated_left = self.update_min_crossroads(
                            self.position,
                            self.direction.clone().turn_left(),
                        );
                        if updated_left {
                            self.crossroads_to_check.push((
                                self.direction.turn_left().next_position(self.position),
                                self.direction.turn_left(),
                                {
                                    let mut path = self.current_path.clone();
                                    path.push(self.direction.turn_left());
                                    path
                                },
                            ));
                        }
                    }
                    if !updated_forward {
                        if updated_right {
                            self.direction = self.direction.turn_right();
                            self.position = self.direction.next_position(self.position);
                            return true;
                        } else if updated_left {
                            self.direction = self.direction.turn_left();
                            self.position = self.direction.next_position(self.position);
                            return true;
                        } else {
                            return false;
                        }
                    }
                    self.position = self.direction.next_position(self.position);
                    self.current_path.push(self.direction.clone());
                    true
                } else {
                    if can_turn_left && can_turn_right {
                        let updated_left = self.update_min_crossroads(
                            self.position,
                            self.direction.clone().turn_left(),
                        );
                        let updated_right = self.update_min_crossroads(
                            self.position,
                            self.direction.clone().turn_right(),
                        );
                        if updated_left && updated_right {
                            self.crossroads_to_check.push((
                                self.direction.turn_left().next_position(self.position),
                                self.direction.turn_left(),
                                {
                                    let mut path = self.current_path.clone();
                                    path.push(self.direction.clone().turn_left());
                                    path
                                },
                            ));
                        }
                        if updated_right {
                            self.direction = self.direction.turn_right();
                            self.position = self.direction.next_position(self.position);
                            true
                        } else if updated_left {
                            self.direction = self.direction.turn_left();
                            self.position = self.direction.next_position(self.position);
                            true
                        } else {
                            return false;
                        }
                    } else {
                        if can_turn_right {
                            self.direction = self.direction.turn_right();
                            self.position = self.direction.next_position(self.position);
                            self.current_path.push(self.direction.clone());
                            return true;
                        }
                        if can_turn_left {
                            self.direction = self.direction.turn_left();
                            self.position = self.direction.next_position(self.position);
                            self.current_path.push(self.direction.clone());
                            return true;
                        }
                        false
                    }
                }
            }
            'E' => {
                self.current_path.push(self.direction.clone());
                let current_score = Reindeer::calculate_path_score(&self.current_path);
                if self.current_min_path.1 > current_score {
                    self.current_min_path.0 = self.current_path.clone();
                    self.current_min_path.1 = current_score;
                }
                false
            }
            'S' => true,
            _ => {
                eprintln!(
                    "Found {} on the map!",
                    self.map[self.position.0][self.position.1]
                );
                unreachable!("There was an unexpected symbol on the map")
            }
        }
    }

    fn find_path_to_finish(&mut self) -> Option<(Vec<Direction>, u32)> {
        self.map[self.position.0][self.position.1] = '.';
        loop {
            println!("{:?}", self);
            self._print_map();
            while self.investigate_next_square() {
                println!("{:?}", self);
                self._print_map();
                thread::sleep(Duration::from_millis(2000));
            }
            println!("Starting a new path!");
            if let Some((position, direction, path)) = self.crossroads_to_check.pop() {
                self.direction = direction;
                self.position = position;
                self.current_path = path;
                self.current_path.push(self.direction.clone());
            } else {
                break;
            }
            thread::sleep(Duration::from_millis(2000));
        }
        if self.current_min_path.1 != 0 {
            println!(
                "Min path = {:?} with score {}",
                self.current_min_path.0, self.current_min_path.1
            );
            Some(self.current_min_path.clone())
        } else {
            None
        }
    }

    fn update_min_crossroads(&mut self, position: (usize, usize), direction: Direction) -> bool {
        let mut inserted = false;

        let mut current_path = self.current_path.clone();
        if self.current_path.len() > 0 {
            *current_path.last_mut().unwrap() = direction.clone();
        } else {
            current_path.push(direction.clone())
        }

        let current_path_clone = current_path.clone();
        let direction_clone = direction.clone();

        let hash_map_crossroad = self
            .crossroads_min_paths
            .entry(position)
            .or_insert_with(|| {
                let mut crossroad_map = HashMap::new();
                inserted = true;
                crossroad_map.insert(direction_clone, current_path_clone);
                // println!("Inserted new map");
                crossroad_map
            });

        let min_path_to_crossroad_with_same_dir =
            hash_map_crossroad.entry(direction).or_insert_with(|| {
                inserted = true;
                // println!("Inserted new value in a map");
                current_path.clone()
            });
        let current_min_score =
            Reindeer::calculate_path_score(&min_path_to_crossroad_with_same_dir);
        let current_path_score = Reindeer::calculate_path_score(&self.current_path);
        if current_min_score > current_path_score {
            *min_path_to_crossroad_with_same_dir = current_path.clone();
            return true;
        } else {
            return inserted;
        }
    }

    fn calculate_path_score(path: &Vec<Direction>) -> u32 {
        if path.len() == 1 {
            0
        } else {
            let mut score = 0;
            for window in path.windows(2) {
                let first = &(*window)[0];
                let second = &(*window)[1];
                if *first == *second {
                    score += 1;
                } else {
                    if first.turn_around() == *second {
                        score += 2000;
                    } else {
                        score += 1000;
                    }
                }
            }
            score
        }
    }

    fn _print_map(&self) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if self.position == (i, j) {
                    print!("@");
                } else {
                    print!("{c}");
                }
            }
            println!("");
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut reindeer = Reindeer::init(&map).unwrap();
    reindeer._print_map();
    let (_path, score) = reindeer.find_path_to_finish().unwrap();
    Some(score)
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
    fn test_path_score() {
        let path_score = Reindeer::calculate_path_score(&vec![Right]);
        assert_eq!(path_score, 0);
    }

    #[test]
    fn test_path_turn() {
        let path_score = Reindeer::calculate_path_score(&vec![Right, Up]);
        assert_eq!(path_score, 1000);
    }

    #[test]
    fn test_path_continue_turn() {
        let path_score = Reindeer::calculate_path_score(&vec![Right, Up, Up]);
        assert_eq!(path_score, 1001);
    }

    #[test]
    fn test_path_turn_continue() {
        let path_score = Reindeer::calculate_path_score(&vec![Right, Up, Up]);
        assert_eq!(path_score, 1001);
    }

    #[test]
    fn test_path_turn_continue_turn() {
        let path_score = Reindeer::calculate_path_score(&vec![Right, Up, Up, Left]);
        assert_eq!(path_score, 2001);
    }
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
