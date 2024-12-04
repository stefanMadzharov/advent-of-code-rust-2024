advent_of_code::solution!(4);
use Direction::*;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
    BOTTOM,
    TOP,
    TOPLEFT,
    TOPRIGHT,
    BOTTOMLEFT,
    BOTTOMRIGHT,
}

impl Direction {
    fn to_absolute_coordinates_xmas(&self, origin: (i32, i32)) -> [(i32, i32); 3] {
        match *self {
            LEFT => [
                (origin.0, origin.1 - 1),
                (origin.0, origin.1 - 2),
                (origin.0, origin.1 - 3),
            ],
            RIGHT => [
                (origin.0, origin.1 + 1),
                (origin.0, origin.1 + 2),
                (origin.0, origin.1 + 3),
            ],
            BOTTOM => [
                (origin.0 + 1, origin.1),
                (origin.0 + 2, origin.1),
                (origin.0 + 3, origin.1),
            ],
            TOP => [
                (origin.0 - 1, origin.1),
                (origin.0 - 2, origin.1),
                (origin.0 - 3, origin.1),
            ],
            TOPLEFT => [
                (origin.0 - 1, origin.1 - 1),
                (origin.0 - 2, origin.1 - 2),
                (origin.0 - 3, origin.1 - 3),
            ],
            TOPRIGHT => [
                (origin.0 - 1, origin.1 + 1),
                (origin.0 - 2, origin.1 + 2),
                (origin.0 - 3, origin.1 + 3),
            ],
            BOTTOMLEFT => [
                (origin.0 + 1, origin.1 - 1),
                (origin.0 + 2, origin.1 - 2),
                (origin.0 + 3, origin.1 - 3),
            ],
            BOTTOMRIGHT => [
                (origin.0 + 1, origin.1 + 1),
                (origin.0 + 2, origin.1 + 2),
                (origin.0 + 3, origin.1 + 3),
            ],
        }
    }
    fn to_absolute_coordinates_x_mas(&self, center: (i32, i32)) -> [(i32, i32); 3] {
        match *self {
            TOPLEFT => [
                (center.0 + 1, center.1 + 1),
                center,
                (center.0 - 1, center.1 - 1),
            ],
            TOPRIGHT => [
                (center.0 + 1, center.1 - 1),
                center,
                (center.0 - 1, center.1 + 1),
            ],
            BOTTOMLEFT => [
                (center.0 - 1, center.1 + 1),
                center,
                (center.0 + 1, center.1 - 1),
            ],
            BOTTOMRIGHT => [
                (center.0 - 1, center.1 - 1),
                center,
                (center.0 + 1, center.1 + 1),
            ],
            _ => [center; 3],
        }
    }

    fn check_out_of_bounds_coordinates(
        coordinates: &[(i32, i32); 3],
        max_lines: i32,
        max_length_column: i32,
    ) -> bool {
        for coordinate in coordinates {
            if coordinate.0 >= max_lines
                || coordinate.0 < 0
                || coordinate.1 >= max_length_column
                || coordinate.1 < 0
            {
                return false;
            }
        }
        true
    }

    fn check_xmas(&self, text: Vec<Vec<char>>, origin: (i32, i32)) -> bool {
        let mut letters: Vec<char> = vec![];
        let coordinates = self.to_absolute_coordinates_xmas(origin);
        if Direction::check_out_of_bounds_coordinates(
            &coordinates,
            text.len() as i32,
            text[0].len() as i32,
        ) {
            for coordinate in coordinates {
                letters.push(text[coordinate.0 as usize][coordinate.1 as usize]);
            }
            letters == vec!['M', 'A', 'S']
        } else {
            false
        }
    }

    fn check_x_mas_diagonal(&self, text: Vec<Vec<char>>, center: (i32, i32)) -> bool {
        let mut letters: Vec<char> = vec![];
        let coordinates = self.to_absolute_coordinates_x_mas(center);
        if Direction::check_out_of_bounds_coordinates(
            &coordinates,
            text.len() as i32,
            text[0].len() as i32,
        ) {
            for coordinate in coordinates {
                letters.push(text[coordinate.0 as usize][coordinate.1 as usize]);
            }
            letters == vec!['M', 'A', 'S']
        } else {
            false
        }
    }
}

const DIRECTION: [Direction; 8] = [
    LEFT,
    RIGHT,
    BOTTOM,
    TOP,
    TOPLEFT,
    TOPRIGHT,
    BOTTOMLEFT,
    BOTTOMRIGHT,
];

pub fn part_one(input: &str) -> Option<u32> {
    let text = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let count = text
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, letter)| {
                    if *letter == 'X' {
                        DIRECTION
                            .iter()
                            .map(|direction| {
                                direction.check_xmas(text.clone(), (i as i32, j as i32))
                            })
                            .filter(|bool| *bool)
                            .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u32;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let text = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let count = text
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, letter)| {
                    if *letter == 'A'
                        && DIRECTION
                            .iter()
                            .map(|direction| {
                                direction.check_x_mas_diagonal(text.clone(), (i as i32, j as i32))
                            })
                            .filter(|bool| *bool)
                            .count()
                            == 2
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u32;
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
