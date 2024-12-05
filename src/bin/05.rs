advent_of_code::solution!(5);
use std::collections::HashMap;

fn parse(input: &str) -> (String, String) {
    let mut rules = String::new();
    let mut updates = String::new();
    let mut found_empty = false;
    for line in input.lines() {
        if line.is_empty() {
            found_empty = true;
        }
        if !found_empty {
            rules = format!("{}\n{}", rules, line);
        } else {
            updates = format!("{}\n{}", updates, line);
        }
    }
    (rules.trim().to_owned(), updates.trim().to_owned())
}

fn parse_hash_map(rules: String) -> HashMap<u32, Vec<u32>> {
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new(); // maps to pages which are before the key
    rules
        .lines()
        .map(|line| {
            let rule: Vec<&str> = line.split('|').collect();
            (
                rule[0].parse::<u32>().unwrap(),
                rule[1].parse::<u32>().unwrap(),
            )
        })
        .for_each(|(prev, after)| {
            before
                .entry(after)
                .and_modify(|vec| vec.push(prev))
                .or_insert(vec![prev]);
        });
    before
}
fn parse_update(update: &str) -> Vec<u32> {
    update
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect::<Vec<u32>>()
}

fn check_update(update: &Vec<u32>, before: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, prev) in update.iter().enumerate() {
        for after in update[i + 1..].iter() {
            if let Some(previous_numbers) = before.get(after) {
                if !previous_numbers.contains(prev) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn order_update(update: Vec<u32>, before: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut update = update
        .iter()
        .map(|page| {
            (
                *page,
                before
                    .iter()
                    .filter(|(key, vec)| update.contains(key) && vec.contains(page))
                    .count(),
            )
        })
        .collect::<Vec<(u32, usize)>>();
    update.sort_by(|(_, a), (_, b)| a.cmp(b).reverse());
    update.iter().map(|(key, _)| *key).collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    let before = parse_hash_map(rules);
    let result = updates
        .lines()
        .map(|update| parse_update(update))
        .filter(|update| check_update(update, &before))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    let before = parse_hash_map(rules);
    let result = updates
        .lines()
        .map(|update| parse_update(update))
        .filter(|update| !check_update(update, &before))
        .map(|update| order_update(update, &before))
        .map(|update| update[update.len() / 2])
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
