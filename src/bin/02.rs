advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|string| string.parse::<u32>().expect("Could not parse the string"))
                .collect::<Vec<u32>>()
        })
        .filter(|report| {
            let mut report = report.iter().peekable();
            let mut increasing_set = false;
            let mut increasing = false;
            while let Some(&level) = report.next() {
                if let Some(&&next) = report.peek() {
                    if !increasing_set {
                        if level != next {
                            increasing = level < next;
                            increasing_set = true;
                            continue;
                        }
                    }
                    let diff: i32 = if increasing {
                        next as i32 - level as i32
                    } else {
                        level as i32 - next as i32
                    };
                    if diff <= 0 || diff > 3 {
                        return false;
                    }
                } else {
                    return true;
                }
            }
            true
        })
        .count() as u32;
    // .collect::<Vec<Vec<u32>>>();
    // println!("{count:?}");
    // None
    Some(count)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
