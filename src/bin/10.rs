use core::num;
use std::u8;

advent_of_code::solution!(10);
use Direction::*;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_value(&self, position: (usize, usize), map: &Vec<Vec<u8>>) -> Option<u8> {
        let (i, j) = position;
        match *self {
            Up => map.get(i.checked_sub(1)?)?.get(j).copied(),
            Down => map.get(i + 1)?.get(j).copied(),
            Left => map[i].get(j.checked_sub(1)?).copied(),
            Right => map[i].get(j + 1).copied(),
        }
    }
}

fn find_trails(topographic_map: &Vec<Vec<u8>>) -> u64 {
    let mut trail_map = topographic_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|&byte| if byte == 9 { 1 } else { 0 })
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();
    for current_digit in (0..9).rev() {
        for (i, row) in topographic_map.iter().enumerate() {
            for (j, digit) in row.iter().enumerate() {
                let mut number_of_paths = 0;
                let directions = vec![Up, Down, Right, Left];
                if *digit == current_digit {
                    for direction in directions {
                        if direction
                            .get_value((i, j), topographic_map)
                            .unwrap_or(u8::MAX)
                            == current_digit + 1
                        {
                            number_of_paths += direction.get_value((i, j), &trail_map).unwrap_or(0);
                        }
                    }
                    trail_map[i][j] = number_of_paths;
                }
            }
        }
        print_map(&trail_map);
    }
    topographic_map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, byte)| **byte == 0)
                .map(|(j, _)| trail_map[i][j] as u64)
                .sum::<u64>()
        })
        .sum()
}

fn print_map(map: &Vec<Vec<u8>>) {
    for row in map {
        for c in row {
            if *c > 9 {
                print!("|{c}|");
            } else {
                print!("{c}");
            }
        }
        println!("");
    }
    println!("");
}

pub fn part_one(input: &str) -> Option<u64> {
    let topographic_map = input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - 48).collect())
        .collect::<Vec<Vec<u8>>>();
    Some(find_trails(&topographic_map))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
