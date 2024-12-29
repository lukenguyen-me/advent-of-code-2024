use std::collections::HashMap;

advent_of_code::solution!(11);

fn traverse(number: u64, level: usize, memo: &mut HashMap<(u64, usize), usize>) -> usize {
    if memo.contains_key(&(number, level)) {
        return memo[&(number, level)];
    }
    if level == 75 {
        return 1;
    }
    let mut result = 0;
    if number == 0 {
        result += traverse(1, level + 1, memo);
    } else if number.to_string().len() % 2 == 0 {
        let y = number.to_string();
        let mid = y.len() / 2;
        result += traverse(y[..mid].parse().unwrap(), level + 1, memo);
        result += traverse(y[mid..].parse().unwrap(), level + 1, memo);
    } else {
        result += traverse(number * 2024, level + 1, memo);
    }
    memo.insert((number, level), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stack: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        let mut new_stack: Vec<u64> = vec![];
        for &x in stack.iter() {
            if x == 0 {
                new_stack.push(1);
            } else if x.to_string().len() % 2 == 0 {
                let y = x.to_string();
                let mid = y.len() / 2;
                new_stack.push(y[..mid].parse().unwrap());
                new_stack.push(y[mid..].parse().unwrap());
            } else {
                new_stack.push(x * 2024);
            }
        }
        stack = new_stack;
    }

    Some(stack.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stack: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    let mut memo: HashMap<(u64, usize), usize> = HashMap::new();
    let mut result = 0;

    for x in stack.iter() {
        result += traverse(*x, 0, &mut memo);
    }

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
