advent_of_code::solution!(2);

fn apply_algo(report: &Vec<u32>) -> bool {
    let mut increasing = false;
    if report[0] > report[1] {
        increasing = false;
    } else if report[0] < report[1] {
        increasing = true;
    } else {
        return false;
    };
    let mut sorted_report = report.clone();
    if increasing {
        sorted_report.sort();
    } else {
        sorted_report.sort_by(|a, b| a.cmp(b).reverse());
    }
    if *report != sorted_report {
        return false;
    }

    let mut prev = report[0];
    for i in 1..report.len() {
        let diff = prev.abs_diff(report[i]);
        if diff == 0 || diff > 3 {
            return false;
        }
        prev = report[i];
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|string| string.parse::<u32>().expect("Could not parse the string"))
                .collect::<Vec<u32>>()
        })
        .filter(|report| apply_algo(report))
        .count() as u32;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|string| string.parse::<u32>().expect("Could not parse the string"))
                .collect::<Vec<u32>>()
        })
        .filter(|report| {
            for i in 0..report.len() {
                let mut report_clone = report.clone();
                report_clone.remove(i);
                if apply_algo(&report_clone) {
                    return true;
                }
            }
            false
        })
        .count() as u32;
    Some(count)
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
        assert_eq!(result, Some(4));
    }
}
