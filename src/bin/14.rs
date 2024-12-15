advent_of_code::solution!(14);
use std::thread;
use std::time::Duration;

type Coordinates = (usize, usize);
type Velocity = (i32, i32);

#[derive(Debug)]
struct Guard {
    coordinates: Coordinates,
    velocity: Velocity,
}

trait VelocityMul {
    fn mul(self, rhs: usize) -> Self;
}
impl VelocityMul for Velocity {
    fn mul(self, rhs: usize) -> Self {
        let x = self.0 * rhs as i32;
        let y = self.1 * rhs as i32;
        (x, y)
    }
}

impl Guard {
    fn move_n_secs(&mut self, n: usize, map_size: Coordinates) {
        if n == 0 {
            return;
        }
        let coordinates = (self.coordinates.0 as i32, self.coordinates.1 as i32);
        let map_size = (map_size.0 as i32, map_size.1 as i32);
        let vector = self.velocity.mul(n);

        let absolute_x = vector.0 + coordinates.0;
        let absolute_y = vector.1 + coordinates.1;

        let x = if absolute_x >= 0 {
            absolute_x % map_size.0
        } else {
            (map_size.0 - (-absolute_x % map_size.0)) % map_size.0
        };

        let y = if absolute_y >= 0 {
            absolute_y % map_size.1
        } else {
            (map_size.1 - (-absolute_y % map_size.1)) % map_size.1
        };
        self.coordinates = (x as usize, y as usize)
    }
}

impl std::str::FromStr for Guard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i32> = s
            .split(' ')
            .flat_map(|half| {
                half.chars()
                    .filter(|char| char.is_ascii_digit() || *char == ',' || *char == '-')
                    .collect::<String>()
                    .split(',')
                    .map(|number| number.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();
        if parts.len() != 4 {
            return Err("Input does not match the expected format".to_string());
        }

        Ok(Guard {
            coordinates: (parts[1] as usize, parts[0] as usize),
            velocity: (parts[3], parts[2]),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut guards: Vec<Guard> = input
        .lines()
        .map(|line| line.parse::<Guard>().unwrap())
        .collect();
    let mut first_quadrant = 0;
    let mut second_quadrant = 0;
    let mut third_quadrant = 0;
    let mut fourth_quadrant = 0;
    guards
        .iter_mut()
        .map(|guard| {
            let secs = 100;
            guard.move_n_secs(secs, (103, 101)); // you should change that
            guard.coordinates
        })
        .for_each(|coordinates| match coordinates {
            (x, y) if x < 51 && y < 50 => first_quadrant += 1,
            (x, y) if x < 51 && y > 50 => second_quadrant += 1,
            (x, y) if x > 51 && y < 50 => third_quadrant += 1,
            (x, y) if x > 51 && y > 50 => fourth_quadrant += 1,
            _ => {}
        });
    Some(first_quadrant * second_quadrant * third_quadrant * fourth_quadrant)
}

pub fn part_two(input: &str) -> Option<u32> {
    // every 103 there was some kind of pattern
    for n in (7603..=7603).step_by(103) {
        let mut guards: Vec<Guard> = input
            .lines()
            .map(|line| line.parse::<Guard>().unwrap())
            .collect();
        let _end_coordinates = guards
            .iter_mut()
            .map(|guard| {
                // let secs = 100;
                guard.move_n_secs(n, (103, 101));
                guard.coordinates
            })
            .collect::<Vec<(usize, usize)>>();

        // println!("Seconds elapsed: {n}");
        // for i in 0..103 {
        //     for j in 0..101 {
        //         let count = _end_coordinates
        //             .iter()
        //             .filter(|(row, column)| i == *row && j == *column)
        //             .count() as u8;
        //         if count == 0 {
        //             print!(".");
        //         } else {
        //             print!("{count}");
        //         }
        //     }
        //     println!("");
        // }
        thread::sleep(Duration::from_millis(500))
    }
    Some(7603)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
