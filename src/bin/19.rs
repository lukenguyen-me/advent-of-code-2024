advent_of_code::solution!(19);

use std::collections::HashMap;

use trie_rs::{Trie, TrieBuilder};

fn check_possible(
    memo: &mut HashMap<String, bool>,
    towers: &Trie<u8>,
    max_len: &usize,
    design: &str,
) -> bool {
    if design.len() == 0 {
        return true;
    }
    if memo.contains_key(design) {
        return memo[design];
    }
    for i in 1..=*max_len {
        if design.len() < i {
            continue;
        }
        if !towers.exact_match(&design[0..i].as_bytes()) {
            continue;
        }

        let result = check_possible(memo, towers, max_len, &design[i..]);
        memo.insert(design.to_string(), result);
        if result {
            return true;
        }
    }

    false
}

fn get_possible_count(
    memo: &mut HashMap<String, u64>,
    towers: &Trie<u8>,
    max_len: &usize,
    design: &str,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }
    if memo.contains_key(design) {
        return memo[design];
    }
    let mut result: u64 = 0;
    for i in 1..=*max_len {
        if design.len() < i {
            continue;
        }
        if !towers.exact_match(&design[0..i].as_bytes()) {
            continue;
        }

        let count = get_possible_count(memo, towers, max_len, &design[i..]);
        result += count;
    }

    memo.insert(design.to_string(), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let (tower_input, designes_input) = input.split_once("\n\n").unwrap();
    let towers: Vec<&str> = tower_input.split(", ").collect();

    let designs: Vec<&str> = designes_input.lines().collect();

    let mut trie = TrieBuilder::new();
    for tower in towers.clone() {
        trie.push(tower);
    }
    let trie = trie.build();

    let mut max_len: usize = 0;
    for tower in towers.clone() {
        if tower.len() > max_len {
            max_len = tower.len();
        }
    }

    let mut memo: HashMap<String, bool> = HashMap::new();

    let mut result: u64 = 0;
    for design in designs {
        if check_possible(&mut memo, &trie, &max_len, design) {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (tower_input, designes_input) = input.split_once("\n\n").unwrap();
    let towers: Vec<&str> = tower_input.split(", ").collect();

    let designs: Vec<&str> = designes_input.lines().collect();

    let mut trie = TrieBuilder::new();
    for tower in towers.clone() {
        trie.push(tower);
    }
    let trie = trie.build();

    let mut max_len: usize = 0;
    for tower in towers.clone() {
        if tower.len() > max_len {
            max_len = tower.len();
        }
    }

    let mut result: u64 = 0;
    for design in designs {
        let mut memo: HashMap<String, u64> = HashMap::new();
        result += get_possible_count(&mut memo, &trie, &max_len, design)
    }

    Some(result)
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
        assert_eq!(result, Some(16));
    }
}
