advent_of_code::solution!(11);
use std::collections::HashMap;

fn num_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let num_digits = num_digits(stone);
        if num_digits % 2 == 0 {
            vec![
                stone / 10_u64.pow(num_digits / 2),
                stone % (10_u64.pow(num_digits / 2)),
            ]
        } else {
            vec![stone * 2024]
        }
    }
}

fn blink_n_times(stones: &Vec<u64>, blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        stones.len() as u64
    } else {
        stones
            .iter()
            .map(|stone| match cache.get_key_value(&(*stone, blinks)) {
                Some((_, value)) => *value,
                None => {
                    let stones = blink(*stone);
                    let lenght = blink_n_times(&stones, blinks - 1, cache);
                    *cache.entry((*stone, blinks)).or_insert(lenght)
                }
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_stones = input
        .split(' ')
        .map(|number| number.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    Some(blink_n_times(&initial_stones, 25, &mut cache))
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_stones = input
        .split(' ')
        .map(|number| number.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    Some(blink_n_times(&initial_stones, 75, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let initial_stones = &advent_of_code::template::read_file("examples", DAY)
            .split(' ')
            .map(|number| number.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
        let result = blink_n_times(&initial_stones, 25, &mut cache);
        assert_eq!(result, 55312);
    }
}
