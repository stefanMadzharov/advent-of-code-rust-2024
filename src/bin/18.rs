advent_of_code::solution!(16);
use std::cmp::{Ord, Ordering};
use Direction::*;

type Position = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd)]
struct PathScore {
    path: Vec<Position>,
    end_position: Position,
}

impl PathScore {
    fn score(&self) -> u32 {
        self.current_cost() + self.heuristic()
    }

    fn current_cost(&self) -> u32 {
        self.path.len() as u32
    }

    fn heuristic(&self) -> u32 {
        let current_position = self.path[self.path.len() - 1];
        (current_position.0.abs_diff(self.end_position.0)
            + current_position.1.abs_diff(self.end_position.1)) as u32
    }
}

impl Ord for PathScore {
    // Required method
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }

    // Provided methods
    fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Equal => {
                if self.path.len() >= other.path.len() {
                    self
                } else {
                    other
                }
            }
            Ordering::Greater => self,
            Ordering::Less => other,
        }
    }
    fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Equal => {
                if self.path.len() < other.path.len() {
                    self
                } else {
                    other
                }
            }
            Ordering::Greater => other,
            Ordering::Less => self,
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        match self.cmp(&min) {
            Ordering::Less => min,
            Ordering::Equal => self,
            Ordering::Greater => match self.cmp(&max) {
                Ordering::Less => self,
                Ordering::Equal => self,
                Ordering::Greater => max,
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
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

#[derive(Clone)]
struct Searcher {
    map: Vec<Vec<char>>,
    position: (usize, usize),
    current_path: Vec<Position>,
    finished: bool,
    priority_que: Vec<((usize, usize), Direction, usize)>,
}

impl Searcher {
    fn init(map: &Vec<Vec<char>>) -> Option<Self> {
        for (i, row) in map.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == 'S' {
                    return None;
                }
            }
        }
        None
    }

    // fn can_turn_left(&self) -> bool {
    //     let (i, j) = self.direction.turn_left().next_position(self.position);
    //     self.map[i][j] == '.'
    // }

    // fn can_turn_right(&self) -> bool {
    //     let (i, j) = self.direction.turn_right().next_position(self.position);
    //     self.map[i][j] == '.'
    // }

    // fn can_continue_forward(&self) -> bool {
    //     let (i, j) = self.direction.next_position(self.position);
    //     self.map[i][j] == '.'
    // }

    // fn find_path_to_finish(&mut self) -> Option<(Vec<Direction>, u32)> {
    //     while !self.finished {
    //         while let Some(is_last_square) = self.investigate_next_square() {
    //             if is_last_square {
    //                 let new_path_value = Reindeer::calculate_path_score(&self.current_path);
    //                 if new_path_value < self.current_min_path.1 {
    //                     self.current_min_path = (self.current_path.clone(), new_path_value);
    //                 }
    //             }
    //             println!("{:?}", self);
    //             // thread::sleep(Duration::from_millis(600));
    //         }
    //         if let Some(crossroad) = self.crossroads_to_check.pop() {
    //             (self.position, self.direction) = (crossroad.0, crossroad.1);
    //             self.current_path = self.current_path[0..crossroad.2].to_vec();
    //         }
    //     }
    //     if self.current_min_path.1 != 0 {
    //         Some(self.current_min_path.clone())
    //     } else {
    //         None
    //     }
    // }

    // fn update_min_crossroads(
    //     &mut self,
    //     position: (usize, usize),
    //     direction: Direction,
    //     deadend: bool,
    // ) -> bool {
    //     let mut inserted = false;

    //     let mut current_path = self.current_path.clone();
    //     if !deadend {
    //         current_path.push(direction.clone());
    //     } else {
    //         current_path
    //             .iter_mut()
    //             .last()
    //             .map(|dir| *dir = direction.clone());
    //     }

    //     let current_path_clone = current_path.clone();
    //     let direction_clone = direction.clone();

    //     let hash_map_crossroad = self
    //         .crossroads_min_paths
    //         .entry(position)
    //         .or_insert_with(|| {
    //             let mut crossroad_map = HashMap::new();
    //             crossroad_map.insert(direction_clone, current_path_clone);
    //             println!("Inserted new map");
    //             crossroad_map
    //         });

    //     let min_path_to_crossroad_with_same_end =
    //         hash_map_crossroad.entry(direction).or_insert_with(|| {
    //             inserted = true;
    //             println!("Inserted new value in a map");
    //             current_path.clone()
    //         });
    //     if self.current_path.len() > 9 {
    //         println! {"Current path: {:?}", current_path};
    //         println! {"Min path to crossroad with same end: {:?}", min_path_to_crossroad_with_same_end};
    //     }
    //     let current_min_score =
    //         Reindeer::calculate_path_score(&min_path_to_crossroad_with_same_end);
    //     let current_path_score = Reindeer::calculate_path_score(&self.current_path);
    //     if current_min_score > current_path_score {
    //         *min_path_to_crossroad_with_same_end = current_path.clone();
    //         return true;
    //     } else {
    //         return inserted;
    //     }
    // }

    // fn calculate_path_score(path: &Vec<Direction>) -> u32 {
    //     let mut current_direction = Right;
    //     let mut score = 0;
    //     for direction in path.iter() {
    //         if *direction == current_direction {
    //             score += 1;
    //         } else {
    //             score += 1000;
    //             current_direction = direction.clone();
    //         }
    //     }
    //     score
    // }
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
    // let mut reindeer = Searcher::init(&map).unwrap();
    // let (_path, score) = reindeer.find_path_to_finish().unwrap();
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
