use std::collections::{hash_map::Entry, HashMap};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let available_patterns = get_available_patterns(input);
    println!("{available_patterns:?}");
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_available_patterns(input: &str) -> HashMap<char, Vec<Vec<char>>> {
    let mut available_patterns: HashMap<char, Vec<Vec<char>>> = HashMap::new();
    let patterns: Vec<&str> = input
        .lines()
        .take(1)
        .flat_map(|line| line.split(',').map(|pattern| pattern.trim()))
        .collect();
    for pattern in patterns {
        let pattern_as_chars = pattern.chars().collect::<Vec<char>>();
        let first_letter = pattern_as_chars[0];
        match available_patterns.entry(first_letter) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().push(pattern_as_chars);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(vec![pattern_as_chars]);
            }
        }
    }
    available_patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
