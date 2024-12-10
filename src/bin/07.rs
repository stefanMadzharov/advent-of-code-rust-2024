advent_of_code::solution!(7);
use itertools::Itertools;
use std::{str::FromStr, string::ParseError};
use Operators::*;

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

#[derive(Debug)]
enum Operators {
    Add,
    Mul,
    Concat,
}

impl FromStr for Equation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let result = parts[0].trim().parse::<u64>().unwrap();
        let numbers = parts[1]
            .trim()
            .split(" ")
            .map(|number| number.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        Ok(Equation { result, numbers })
    }
}

impl Equation {
    fn check(&self, operators: Vec<Operators>) -> bool {
        // let operators = ;
        let iterators = vec![operators.iter(); self.numbers.len() - 1];
        iterators
            .into_iter()
            .multi_cartesian_product()
            .any(|permutation| {
                self.numbers.iter().skip(1).zip(permutation).fold(
                    self.numbers[0],
                    |acc, (number, operator)| match operator {
                        Add => acc + number,
                        Mul => acc * number,
                        Concat => format!("{acc}{number}").parse::<u64>().unwrap(),
                    },
                ) == self.result
            })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<Equation>().unwrap())
            .filter(|equation| equation.check(vec![Add, Mul]))
            .map(|equation| equation.result)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<Equation>().unwrap())
            .filter(|equation| equation.check(vec![Add, Mul, Concat]))
            .map(|equation| equation.result)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
