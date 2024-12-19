advent_of_code::solution!(18);
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::BinaryHeap;
use Direction::*;

type Position = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Clone)]
struct Path {
    path: Vec<Position>,
    end_position: Position,
}

impl Path {
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

impl Ord for Path {
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
    position: Position,
    current_path: Vec<Position>,
    finished: bool,
    priority_queue: BinaryHeap<Reverse<Path>>,
}

const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

impl Searcher {
    fn init(map: &Vec<Vec<char>>) -> Self {
        let mut priority_queue = BinaryHeap::new();
        let path = Path {
            path: vec![(0, 0)],
            end_position: (map.len(), map[0].len()),
        };
        priority_queue.push(Reverse(path));
        Searcher {
            map: map.clone(),
            position: (0, 0),
            current_path: vec![],
            finished: false,
            priority_queue: BinaryHeap::new(),
        }
    }

    fn explore_next_path(&mut self) {
        let next_path = self.priority_queue.pop().unwrap();
        self.position = next_path.0.path[next_path.0.path.len() - 1];
        for direction in DIRECTIONS {
            match direction.get(&self.map, self.position) {
                Some((symbol, position)) => {
                    if symbol != '#' && !self.current_path.contains(&position) {
                        let mut path = self.current_path.clone();
                        path.push(position);
                        let path = Path {
                            path,
                            end_position: (self.map.len(), self.map[0].len()),
                        };
                        self.priority_queue.push(Reverse(path));
                    }
                    if position == (self.map.len(), self.map[0].len()) {
                        self.finished = true
                    }
                }
                None => {}
            };
        }
    }

    fn find_path_to_finish(&mut self) -> Vec<Position> {
        while !self.finished {
            self.explore_next_path();
        }
        self.current_path.clone()
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

fn get_free_map(dimensions: usize) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..dimensions {
        map.push(vec!['.'].repeat(dimensions))
    }
    map
}

fn populate_map_with_obstacles(
    map: &mut Vec<Vec<char>>,
    obstacles: Vec<Position>,
    number_of_obstacles: usize,
) {
    for position in obstacles[..number_of_obstacles].iter() {
        map[position.0][position.1] = '#';
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = get_free_map(7);
    _print_map(&map);
    let obstacles: Vec<Position> = input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|coordinates| {
            (
                coordinates[1].trim().parse::<usize>().unwrap(),
                coordinates[0].trim().parse::<usize>().unwrap(),
            )
        })
        .collect();
    populate_map_with_obstacles(&mut map, obstacles, 12);
    _print_map(&map);
    let mut searcher = Searcher::init(&map);
    let path = searcher.find_path_to_finish();
    Some(path.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
