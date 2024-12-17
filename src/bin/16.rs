advent_of_code::solution!(16);
use std::{collections::HashMap, thread::current};
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

    fn next_next_position(&self, position: (usize, usize)) -> (usize, usize) {
        match *self {
            Up => (position.0 - 2, position.1),
            Right => (position.0, position.1 + 2),
            Down => (position.0 + 2, position.1),
            Left => (position.0, position.1 - 2),
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
    map: Vec<Vec<char>>,
    position: (usize, usize),
    direction: Direction,
    current_path: Vec<Direction>,
    finished: bool,
    current_min_path: (Vec<Direction>, u32),
    crossroads_to_check: Vec<((usize, usize), Direction, usize)>,
    crossroads_min_paths: HashMap<(usize, usize), HashMap<Direction, Vec<Direction>>>,
}

impl std::fmt::Debug for Reindeer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "pos: {:?}, dir: {:?},\ncrossroads_to_check: {:?},\ncrossroads_min_paths:  {:?}\n",
            self.position, self.direction, self.crossroads_to_check, self.crossroads_min_paths
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
                        finished: false,
                        crossroads_to_check: vec![((i, j), Up, 1)],
                        crossroads_min_paths: HashMap::new(),
                    });
                }
            }
        }
        None
    }

    fn can_turn_left(&self) -> bool {
        let (i, j) = self.direction.turn_left().next_position(self.position);
        self.map[i][j] == '.'
    }

    fn can_turn_right(&self) -> bool {
        let (i, j) = self.direction.turn_right().next_position(self.position);
        self.map[i][j] == '.'
    }

    fn can_continue_forward(&self) -> bool {
        let (i, j) = self.direction.next_next_position(self.position);
        self.map[i][j] == '.'
    }

    fn investigate_next_square(&mut self) -> Option<bool> {
        let (row, col) = self.direction.next_position(self.position);
        match self.map[row][col] {
            '#' => {
                let mut changed_direction = false;
                let can_turn_right = self.can_turn_right();
                let can_turn_left = self.can_turn_left();
                if can_turn_right {
                    if can_turn_left {
                        let new_direction = self.direction.turn_left();
                        if self.update_min_crossroads(new_direction.clone(), true) {
                            self.crossroads_to_check.push((
                                self.position,
                                new_direction,
                                self.current_path.len(),
                            ));
                        }
                    }
                    changed_direction = true;
                    self.direction = self.direction.turn_right();
                } else {
                    if can_turn_left {
                        self.direction = self.direction.turn_left();
                        changed_direction = true;
                    }
                }
                if changed_direction {
                    Some(false)
                } else {
                    None
                }
            }
            '.' => {
                if self.can_turn_left() && self.can_continue_forward() {
                    let new_direction = self.direction.turn_left();
                    if self.update_min_crossroads(new_direction.clone(), false) {
                        self.crossroads_to_check.push((
                            self.position,
                            new_direction,
                            self.current_path.len(),
                        ));
                    }
                }
                if self.can_turn_right() && self.can_continue_forward() {
                    let new_direction = self.direction.turn_right();
                    if self.update_min_crossroads(new_direction.clone(), false) {
                        self.crossroads_to_check.push((
                            self.position,
                            new_direction,
                            self.current_path.len(),
                        ));
                    }
                }
                self.position = (row, col);
                self.current_path.push(self.direction.clone());
                Some(false)
            }
            'E' => {
                self.current_path.push(self.direction.clone());
                Some(true)
            }
            'S' => None,
            _ => {
                eprintln!("Found {} on the map!", self.map[row][col]);
                unreachable!("There was an unexpected symbol on the map")
            }
        }
    }

    fn find_path_to_finish(&mut self) -> Option<(Vec<Direction>, u32)> {
        while !self.finished {
            while let Some(is_last_square) = self.investigate_next_square() {
                if is_last_square {
                    let new_path_value = Reindeer::calculate_path_score(&self.current_path);
                    if new_path_value < self.current_min_path.1 {
                        self.current_min_path = (self.current_path.clone(), new_path_value);
                    }
                }
                println!("{:?}", self);
            }
            if let Some(crossroad) = self.crossroads_to_check.pop() {
                (self.position, self.direction) = (crossroad.0, crossroad.1);
                self.current_path = self.current_path[0..crossroad.2].to_vec();
            }
        }
        if self.current_min_path.1 != 0 {
            Some(self.current_min_path.clone())
        } else {
            None
        }
    }

    fn update_min_crossroads(&mut self, direction: Direction, deadend: bool) -> bool {
        let mut inserted = false;

        let mut current_path = self.current_path.clone();
        if !deadend {
            current_path.push(direction.clone());
        } else {
            current_path
                .iter_mut()
                .last()
                .map(|dir| *dir = direction.clone());
        }

        let current_path_clone = current_path.clone();
        let direction_clone = direction.clone();

        let hash_map_crossroad = self
            .crossroads_min_paths
            .entry(self.position)
            .or_insert_with(|| {
                let mut crossroad_map = HashMap::new();
                crossroad_map.insert(direction_clone, current_path_clone);
                println!("Inserted new map");
                crossroad_map
            });

        let min_path_to_crossroad_with_same_end =
            hash_map_crossroad.entry(direction).or_insert_with(|| {
                inserted = true;
                println!("Inserted new value in a map");
                current_path.clone()
            });

        if Reindeer::calculate_path_score(&min_path_to_crossroad_with_same_end)
            > Reindeer::calculate_path_score(&self.current_path)
        {
            *min_path_to_crossroad_with_same_end = current_path.clone();
            return true;
        } else {
            return inserted;
        }
    }

    fn calculate_path_score(path: &Vec<Direction>) -> u32 {
        let mut current_direction = Right;
        let mut score = 0;
        for direction in path.iter() {
            if *direction == current_direction {
                score += 1;
            } else {
                score += 1000;
                current_direction = direction.clone();
            }
        }
        score
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
    let mut reindeer = Reindeer::init(&map).unwrap();
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
