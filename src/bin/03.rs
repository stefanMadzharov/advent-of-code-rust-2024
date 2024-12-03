advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        regex
            .captures_iter(input)
            .map(|captures| {
                captures[1].parse::<u32>().unwrap() * captures[2].parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = format!("do(){input}");
    Some(
        input
            .split("do()")
            .skip(1)
            .map(|split| {
                if let Some(pos_dont) = split.find("don't()") {
                    part_one(&split[..pos_dont]).unwrap_or(0)
                } else {
                    part_one(split).unwrap_or(0)
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
