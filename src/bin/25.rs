advent_of_code::solution!(25);

type Lock = [usize; 5];
type Key = [usize; 5];

fn parse_key(input: &str) -> Key {
    let mut key = [0; 5];
    input.lines().skip(1).take(5).for_each(|line| {
        line.chars().enumerate().for_each(|(i, ch)| {
            if ch == '#' {
                key[i] += 1
            }
        })
    });
    key
}

fn parse_lock(input: &str) -> Lock {
    let mut lock = [5; 5];
    input.lines().skip(1).take(5).for_each(|line| {
        line.chars().enumerate().for_each(|(i, ch)| {
            if ch == '.' {
                lock[i] -= 1
            }
        })
    });
    lock
}

fn parse(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks: Vec<Lock> = vec![];
    let mut keys: Vec<Key> = vec![];
    input.split("\n\n").for_each(|input| {
        if input.chars().next() == Some('#') {
            locks.push(parse_lock(input))
        } else {
            keys.push(parse_key(input))
        }
    });
    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse(input);
    Some(
        locks
            .into_iter()
            .map(|lock| {
                keys.iter()
                    .filter(|key| {
                        lock.iter()
                            .zip(key.iter())
                            .all(|(lock, key)| lock + key <= 5)
                    })
                    .count() as u32
            })
            .sum::<u32>(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
