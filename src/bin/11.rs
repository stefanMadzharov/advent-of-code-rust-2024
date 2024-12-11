advent_of_code::solution!(11);
use rayon::prelude::*;

fn num_digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

fn apply_rules(stone: u64) -> Vec<u64> {
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

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| apply_rules(*stone))
        .collect::<Vec<u64>>()
}

fn blink_n_times_to_vec(stones: &Vec<u64>, blinks: u64) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| {
            let mut stones = vec![*stone];
            for _ in 0..blinks {
                stones = blink(&stones);
            }
            stones
        })
        .collect()
}

fn blink_n_times(stones: &Vec<u64>, blinks: u64) -> u64 {
    if blinks < 50 {
        stones
            .par_iter()
            .map(|stone| {
                let mut stones = vec![*stone];
                for _ in 0..blinks {
                    stones = blink(&stones);
                }
                stones.len() as u64
            })
            .sum()
    } else {
        let precompute = 35;
        let stones = blink_n_times_to_vec(stones, precompute);
        let blinks = blinks - precompute;
        println!("Precompute ready!");
        stones
            .par_iter()
            .map(|stone| {
                let mut stones = vec![*stone];
                for _ in 0..blinks {
                    stones = blink(&stones);
                }
                stones.len() as u64
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_stones = input
        .split(' ')
        .map(|number| number.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    Some(blink_n_times(&initial_stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_stones = input
        .split(' ')
        .map(|number| number.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    Some(blink_n_times(&initial_stones, 75))
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
        let should_be_results: Vec<Vec<u64>> = vec![
            vec![125, 17],
            vec![253000, 1, 7],
            vec![253, 0, 2024, 14168],
            vec![512072, 1, 20, 24, 28676032],
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ],
        ];
        for i in 0..6 {
            let result = blink_n_times_to_vec(initial_stones, i);
            assert_eq!(result, should_be_results[i as usize], "{i}");
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
