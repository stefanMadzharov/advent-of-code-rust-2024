advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let strings = line.split("   ").collect::<Vec<&str>>();
        let el1: u32 = strings[0].parse().expect("El1 could not be parsed");
        let el2: u32 = strings[1].parse().expect("El2 could not be parsed");
        list1.push(el1);
        list2.push(el2);
    }
    return (list1, list2);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = parse(input);
    list1.sort();
    list2.sort();
    let sum = list1
        .iter()
        .zip(list2.iter())
        .map(|(&el1, &el2)| el1.abs_diff(el2))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse(input);
    let sum = list1
        .iter()
        .map(|&el1| list2.iter().filter(|&&el2| el1 == el2).count() as u32 * el1)
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{}", result.unwrap());
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
