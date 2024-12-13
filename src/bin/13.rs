advent_of_code::solution!(13);

#[derive(Debug, Default)]
struct Equation {
    // according to Cramer's rule
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
}

impl Equation {
    fn parse_button_a(&mut self, input: &str) {
        input
            .chars()
            .filter(|char| char.is_ascii_digit() || *char == ',')
            .collect::<String>()
            .split(',')
            .enumerate()
            .map(|(i, value)| {
                if i == 0 {
                    self.b = value.parse().unwrap()
                } else {
                    self.d = value.parse().unwrap()
                }
            })
            .count();
    }

    fn parse_button_b(&mut self, input: &str) {
        input
            .chars()
            .filter(|char| char.is_ascii_digit() || *char == ',')
            .collect::<String>()
            .split(',')
            .enumerate()
            .map(|(i, value)| {
                if i == 0 {
                    self.a = value.parse().unwrap()
                } else {
                    self.c = value.parse().unwrap()
                }
            })
            .count();
    }

    fn parse_prizes(&mut self, input: &str) {
        input
            .chars()
            .filter(|char| char.is_ascii_digit() || *char == ',')
            .collect::<String>()
            .split(',')
            .enumerate()
            .map(|(i, value)| {
                if i == 0 {
                    self.e = value.parse().unwrap()
                } else {
                    self.f = value.parse().unwrap()
                }
            })
            .count();
    }

    fn parse_prizes_part_two(&mut self, input: &str) {
        input
            .chars()
            .filter(|char| char.is_ascii_digit() || *char == ',')
            .collect::<String>()
            .split(',')
            .enumerate()
            .map(|(i, value)| {
                if i == 0 {
                    self.e = value.parse::<i64>().unwrap() + 10_000_000_000_000_i64
                } else {
                    self.f = value.parse::<i64>().unwrap() + 10_000_000_000_000_i64
                }
            })
            .count();
    }

    #[inline]
    fn det_a(&self) -> i64 {
        self.a * self.d - self.b * self.c
    }

    #[inline]
    fn det_a_x(&self) -> i64 {
        self.a * self.f - self.e * self.c
    }

    #[inline]
    fn det_a_y(&self) -> i64 {
        self.e * self.d - self.b * self.f
    }

    #[inline]
    // How many times to press Button A
    fn solve_for_a(&self) -> Option<i64> {
        let det_a = self.det_a();
        if det_a != 0 {
            let det_a_x = self.det_a_x();
            if det_a_x % det_a == 0 {
                Some(det_a_x / det_a)
            } else {
                None
            }
        } else {
            // TODO extend with infinitely many solutions case if needed
            None // No solution case
        }
    }

    #[inline]
    // How many times to press Button B
    fn solve_for_b(&self) -> Option<i64> {
        let det_a = self.det_a();
        if det_a != 0 {
            let det_a_y = self.det_a_y();
            if det_a_y % det_a == 0 {
                Some(self.det_a_y() / det_a)
            } else {
                None
            }
        } else {
            // TODO extend with infinitely many solutions case if needed
            None // No solution case
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut equations = vec![];
    let mut equation = Equation::default();
    for (i, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        match i % 3 {
            0 => equation.parse_button_a(line),
            1 => equation.parse_button_b(line),
            2 => {
                equation.parse_prizes(line);
                equations.push(equation);
                equation = Equation::default();
            }
            _ => unreachable!("Should not be possible"),
        }
    }
    Some(
        equations
            .iter()
            .filter_map(|equation| Some((equation.solve_for_a()?, equation.solve_for_b()?)))
            .filter(|(a, b)| *a <= 100 && *a >= 0 && *b <= 100 && *b >= 0)
            .map(|(a, b)| (a * 3 + b) as u64)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut equations = vec![];
    let mut equation = Equation::default();
    for (i, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        match i % 3 {
            0 => equation.parse_button_a(line),
            1 => equation.parse_button_b(line),
            2 => {
                equation.parse_prizes_part_two(line);
                equations.push(equation);
                equation = Equation::default();
            }
            _ => unreachable!("Should not be possible"),
        }
    }
    Some(
        equations
            .iter()
            .filter_map(|equation| Some((equation.solve_for_a()?, equation.solve_for_b()?)))
            .filter(|(a, b)| *a >= 0 && *b >= 0)
            .map(|(a, b)| (a * 3 + b) as u64)
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
