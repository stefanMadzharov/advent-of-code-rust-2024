advent_of_code::solution!(18);
use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;
use Direction::*;

type Position = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Clone)]
struct Paths {
    path: Vec<Position>,
    end_position: Position,
}

impl Paths {
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

impl Ord for Paths {
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
    priority_queue: BinaryHeap<Paths>,
}

impl Searcher {
    fn init(map: &Vec<Vec<char>>) -> Self {
        Searcher {
            map: map.clone(),
            position: (0, 0),
            current_path: vec![],
            finished: false,
            priority_queue: BinaryHeap::new(),
        }
    }

    fn find_path_to_finish(&mut self) -> Vec<Position> {
        // while !self.finished {}
        vec![]
    } //
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

fn get_free_map(dimensions: u32) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];
    let map_dimensions = 7;
    for _ in 0..map_dimensions {
        map.push(vec!['.'].repeat(map_dimensions))
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
            println!("Coordinates: {coordinates:?}");
            (
                coordinates[1].trim().parse::<usize>().unwrap(),
                coordinates[0].trim().parse::<usize>().unwrap(),
            )
        })
        .collect();
    populate_map_with_obstacles(&mut map, obstacles, 12);
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
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
