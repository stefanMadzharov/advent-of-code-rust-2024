#![feature(iter_array_chunks)]

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Option<u64>> {
    let mut disc_map = Vec::new();
    let mut empty_blocks = false;
    for (i, digit) in input.chars().enumerate() {
        if empty_blocks {
            disc_map.extend(vec![None].repeat(digit.to_string().parse().unwrap()));
        } else {
            disc_map.extend(vec![Some(i as u64 / 2)].repeat(digit.to_string().parse().unwrap()));
        }
        empty_blocks = !empty_blocks;
    }
    disc_map
}

fn fragment(mut disc_map: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut left = 0;
    let mut right = disc_map.len() - 1;
    while left < right {
        if disc_map[left].is_some() {
            left += 1;
        }
        if disc_map[right].is_none() {
            right -= 1;
        }
        if disc_map[left].is_none() && disc_map[right].is_some() {
            disc_map[left] = disc_map[right];
            disc_map[right] = None;
        }
    }
    disc_map
}

fn find_space(needed_space: usize, disc_map: &Vec<Option<u64>>, limit: usize) -> Option<usize> {
    for (i, _) in disc_map
        .iter()
        .enumerate()
        .skip_while(|(_, &block)| block.is_some())
    {
        if i >= limit {
            return None;
        }
        if let Some((index, _)) = disc_map[i..]
            .iter()
            .enumerate()
            .find(|(_, block)| block.is_some())
        {
            if index >= needed_space {
                return Some(i);
            }
        }
    }
    None
}

fn swap(element_count: usize, disc_map: &mut Vec<Option<u64>>, starting_index: usize) {
    if let Some(free_space_index) = find_space(element_count, &disc_map, starting_index) {
        for j in 0..element_count {
            disc_map[free_space_index + j] = disc_map[starting_index + j];
            disc_map[starting_index + j] = None;
        }
    }
}

fn defragment(mut disc_map: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut id_being_processed = disc_map.iter().last().unwrap().unwrap();
    let mut element_count = 0;
    for i in (1..disc_map.len()).rev() {
        match disc_map[i] {
            Some(current_id) => {
                if current_id == id_being_processed {
                    element_count += 1;
                } else {
                    swap(element_count, &mut disc_map, i);
                    element_count = 0;
                }
            }
            None => {
                if element_count > 0 {
                    swap(element_count, &mut disc_map, i + 1);
                    element_count = 0;
                }
            }
        };
        if let Some(next_id) = disc_map[i - 1] {
            if next_id != id_being_processed {
                id_being_processed = next_id;
                swap(element_count, &mut disc_map, i);
                element_count = 0;
            }
        }
    }
    disc_map
}

fn calculate_checksum(disc_map: Vec<Option<u64>>) -> u64 {
    disc_map
        .iter()
        .enumerate()
        .map(|(i, digit)| i as u64 * digit.unwrap_or_default())
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disc_map = parse(input);
    disc_map = fragment(disc_map);
    Some(calculate_checksum(disc_map))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disc_map = parse(input);
    disc_map = defragment(disc_map);
    Some(calculate_checksum(disc_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
