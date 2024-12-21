#![feature(iter_map_windows)]
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(21);

//numpad buttoms
const SEVEN: (u8, u8) = (0, 0);
const EIGHT: (u8, u8) = (0, 1);
const NINE: (u8, u8) = (0, 2);

const FOUR: (u8, u8) = (1, 0);
const FIVE: (u8, u8) = (1, 1);
const SIX: (u8, u8) = (1, 2);

const ONE: (u8, u8) = (2, 0);
const TWO: (u8, u8) = (2, 1);
const THREE: (u8, u8) = (2, 2);

const ZERO: (u8, u8) = (3, 1);
const A_NUMPAD: (u8, u8) = (3, 2);

//keypad buttoms
const UP: (u8, u8) = (0, 1);
const A_KEYPAD: (u8, u8) = (0, 2);
const LEFT: (u8, u8) = (1, 0);
const DOWN: (u8, u8) = (1, 1);
const RIGHT: (u8, u8) = (1, 2);

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn get_all_paths(numeric: bool, from: (u8, u8), to: (u8, u8)) -> Vec<Vec<Direction>> {
        let mut paths = vec![];
        if from.0 > to.0 && (from != (1, 0) || numeric) {
            let mut new_paths = Self::Up.get_paths_from_direction(numeric, from, to);
            paths.append(&mut new_paths);
        }
        if from.0 < to.0 && (from != (2, 0) || !numeric) {
            let mut new_paths = Self::Down.get_paths_from_direction(numeric, from, to);
            paths.append(&mut new_paths);
        }
        if from.1 > to.1 && (from != (3, 1) || !numeric) && (from != (0, 1) || numeric) {
            let mut new_paths = Self::Left.get_paths_from_direction(numeric, from, to);
            paths.append(&mut new_paths);
        }
        if from.1 < to.1 {
            let mut new_paths = Self::Right.get_paths_from_direction(numeric, from, to);
            paths.append(&mut new_paths);
        }
        paths
    }

    fn get_paths_from_direction(
        &self,
        numeric: bool,
        from: (u8, u8),
        to: (u8, u8),
    ) -> Vec<Vec<Direction>> {
        let mut paths = vec![];
        let new_from = match *self {
            Self::Up => (from.0 - 1, from.1),
            Self::Down => (from.0 + 1, from.1),
            Self::Right => (from.0, from.1 + 1),
            Self::Left => (from.0, from.1 - 1),
        };
        let mut new_paths = Self::get_all_paths(numeric, new_from, to);
        if new_paths.is_empty() {
            paths.push(vec![(*self).clone()])
        } else {
            new_paths.iter_mut().for_each(|path| {
                path.insert(0, (*self).clone());
            });
            paths.append(&mut new_paths);
        }
        paths
    }

    fn to_char(&self) -> char {
        match *self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Right => '>',
            Self::Left => '<',
        }
    }
}

fn press_on_pad(needed_input: &str, scoring_map: &HashMap<String, usize>) -> Vec<String> {
    let mut sequences = vec![];
    let is_keypad = ['^', 'v', '>', '<']
        .iter()
        .any(|needle| needed_input.contains(*needle));
    let mut current_button = if is_keypad { A_KEYPAD } else { A_NUMPAD };
    for to in needed_input.chars() {
        let next_button = match to {
            //numpad
            '0' => ZERO,
            '1' => ONE,
            '2' => TWO,
            '3' => THREE,
            '4' => FOUR,
            '5' => FIVE,
            '6' => SIX,
            '7' => SEVEN,
            '8' => EIGHT,
            '9' => NINE,
            //keypad
            '^' => UP,
            'v' => DOWN,
            '>' => RIGHT,
            '<' => LEFT,
            // Special case
            'A' => {
                if is_keypad {
                    A_KEYPAD
                } else {
                    A_NUMPAD
                }
            }
            _ => unreachable!("Wrong input!"),
        };
        let next_button_sequences: Vec<String> =
            Direction::get_all_paths(!is_keypad, current_button, next_button)
                .iter()
                .map(|path| {
                    path.iter()
                        .map(|dir| dir.to_char())
                        .chain(std::iter::once('A'))
                        .collect()
                })
                .collect();
        current_button = next_button;
        let mut temp_sequences = vec![];
        if sequences.is_empty() {
            if next_button_sequences.is_empty() {
                sequences = vec![String::from("A")]
            } else {
                sequences = next_button_sequences;
            }
        } else {
            if next_button_sequences.is_empty() {
                for sequence_to_current_button in &mut sequences {
                    temp_sequences.push(sequence_to_current_button.to_owned() + "A")
                }
            } else {
                for next_button_sequence in &next_button_sequences {
                    for sequence_to_current_button in &sequences {
                        temp_sequences
                            .push(sequence_to_current_button.to_owned() + next_button_sequence)
                    }
                }
            }
            sequences = temp_sequences
        }
    }
    let initial = sequences.len();
    let sequences = sequences
        .iter()
        .min_set_by_key(|sequence| {
            let score = score(&sequence, scoring_map);
            // println!("Overall: {score}\n");
            score
        })
        .into_iter()
        .map(|sequence| sequence.clone())
        .collect::<Vec<String>>();
    println!(
        "Sequences before pruning: {:?}, with length {}\n",
        initial,
        sequences[0].len()
    );
    println!("Sequences after pruning: {:?}\n", sequences.len());
    println!("Sequences pruned: {:?}\n", initial - sequences.len());
    println!(
        "Sequences pruned {:?}%\n",
        ((initial - sequences.len()) as f32 / initial as f32 * 100.0)
    );
    println!("{}", "_".repeat(60));
    sequences
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

fn create_scoring_map() -> HashMap<String, usize> {
    let mut scoring = HashMap::new();
    let chars = vec!['v', '<', '>', '^', 'A'];
    let combinations = (0..2).map(|_| chars.iter()).multi_cartesian_product();
    let empty_map = HashMap::new();
    for combination in combinations {
        let combination = combination.iter().join("");
        let score = press_on_pad(&combination, &empty_map)
            .iter()
            .flat_map(|sequence| press_on_pad(&sequence, &empty_map))
            .min_by_key(|sequence| sequence.len())
            .unwrap()
            .to_owned()
            .len();
        scoring.insert(combination, score);
    }
    // println!("{:?}", scoring);
    scoring
}

fn score(sequence: &str, scoring_map: &HashMap<String, usize>) -> usize {
    sequence
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|window| {
            let string = window.iter().join("");
            let score = scoring_map.get(&string).unwrap_or(&0);
            // println!("Score of {string} is {score}");
            score
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let scoring_map = create_scoring_map();
    // println!("Scoring_map: {scoring_map:?}");
    // press_on_pad("<A^A>", &scoring_map);
    println!("{}", "\n".repeat(30));
    Some(
        input
            .lines()
            .map(|line| {
                line[..3].parse::<usize>().unwrap()
                    * press_on_pad(line, &scoring_map)
                        .iter()
                        .flat_map(|sequence| press_on_pad(&sequence, &scoring_map))
                        .flat_map(|sequence| press_on_pad(&sequence, &scoring_map))
                        .min_by_key(|sequence| sequence.len())
                        .unwrap()
                        .to_owned()
                        .len()
            })
            .sum::<usize>(),
    )
    // None
}

pub fn part_two(input: &str) -> Option<usize> {
    let scoring_map = create_scoring_map();
    println!("{}", "\n".repeat(30));
    Some(
        input
            .lines()
            .map(|line| {
                line[..3].parse::<usize>().unwrap()
                    * press_on_pad(line, &scoring_map)
                        .iter()
                        .flat_map(|sequence| press_on_pad(&sequence, &scoring_map))
                        .flat_map(|sequence| press_on_pad(&sequence, &scoring_map))
                        .flat_map(|sequence| press_on_pad(&sequence, &scoring_map))
                        .min_by_key(|sequence| sequence.len())
                        .unwrap()
                        .to_owned()
                        .len()
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
