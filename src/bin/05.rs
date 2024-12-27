use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(u64, u64)> = orderings
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &u64, y: &u64| !rules.contains(&(*y, *x));

    let mut result: u64 = 0;
    for update in updates.lines() {
        let update: Vec<u64> = update.split(',').map(|x| x.parse().unwrap()).collect();

        if update.is_sorted_by(compare) {
            result += update[update.len() / 2];
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(u64, u64)> = orderings
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &u64, y: &u64| {
        let (x, y) = (*x, *y);
        if rules.contains(&(x, y)) {
            Ordering::Less
        } else if rules.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut result: u64 = 0;
    for update in updates.lines() {
        let mut update: Vec<u64> = update.split(',').map(|x| x.parse().unwrap()).collect();

        if !update.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
            update.sort_by(compare);
            result += update[update.len() / 2];
        }
    }

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
