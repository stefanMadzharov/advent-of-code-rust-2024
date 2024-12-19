advent_of_code::solution!(18);
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::BinaryHeap;
use Direction::*;

type Position = (usize, usize);

#[derive(PartialEq, Eq, Clone, Debug)]
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

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
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

const DIRECTIONS: [Direction; 4] = [Down, Right, Up, Left];

impl Searcher {
    fn init(map: &Vec<Vec<char>>) -> Self {
        let mut priority_queue = BinaryHeap::with_capacity(map.len() * map[0].len() / 3);
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
            priority_queue,
        }
    }

    fn explore_next_path(&mut self) -> bool {
        if let Some(next_path) = self.priority_queue.pop() {
            self.current_path = next_path.0.path.clone();
            self.position = next_path.0.path[next_path.0.path.len() - 1];
            for direction in DIRECTIONS {
                if let Some((symbol, position)) = direction.get(&self.map, self.position) {
                    if symbol != '#' && !self.current_path.contains(&position) {
                        let mut new_path = self.current_path.clone();
                        new_path.push(position);
                        let new_path = Path {
                            path: new_path,
                            end_position: (self.map.len(), self.map[0].len()),
                        };

                        if !self
                            .priority_queue
                            .iter()
                            .any(|path| path.0.path.contains(&position))
                        {
                            self.priority_queue.push(Reverse(new_path));
                        }
                    }
                    if position == (self.map.len() - 1, self.map[0].len() - 1) {
                        self.finished = true;
                    };
                };
            }
            true
        } else {
            false
        }
    }

    fn find_path_to_finish(&mut self) -> Option<Vec<Position>> {
        while !self.finished {
            if !self.explore_next_path() {
                return None;
            }
        }
        Some(self.current_path.clone())
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

fn _print_map_with_path(map: &Vec<Vec<char>>, path: &Vec<Position>) {
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if path.contains(&(i, j)) {
                if *c == '.' {
                    print!("O")
                } else {
                    print!("X")
                }
            } else {
                print!("{c}");
            }
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
    let mut map = get_free_map(71);
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
    populate_map_with_obstacles(&mut map, obstacles, 1024);
    let mut searcher = Searcher::init(&map);
    let path = searcher.find_path_to_finish().unwrap();
    Some(path.len() as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map_orig = get_free_map(71);
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

    let mut searcher = Searcher::init(&map_orig.clone());
    let mut path = searcher.find_path_to_finish().unwrap();
    let mut blocking_byte = 0;
    for (i, obstacle) in obstacles.clone().iter().enumerate() {
        if path.contains(obstacle) {
            let mut map = map_orig.clone();
            populate_map_with_obstacles(&mut map, obstacles.clone(), i + 1);
            let mut searcher = Searcher::init(&map);
            if let Some(new_path) = searcher.find_path_to_finish() {
                path = new_path;
            } else {
                println!("Blocking byte is {:?}", (obstacle.0, obstacle.1));
                blocking_byte = i;
                break;
            }
        }
    }
    Some(blocking_byte)
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
        assert_eq!(result, Some(20));
    }
}
