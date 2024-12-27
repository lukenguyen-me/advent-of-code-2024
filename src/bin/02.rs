advent_of_code::solution!(2);

fn check_safe(levels: &[u64]) -> bool {
    if levels.len() <= 1 {
        return true;
    }

    let order = levels[0].cmp(&levels[1]);

    for window in levels.windows(2) {
        let cur = window.first().unwrap();
        let next = window.get(1).unwrap();

        if cur.cmp(next) != order {
            return false;
        }
        if cur.abs_diff(*next) > 3 || cur.abs_diff(*next) < 1 {
            return false;
        }
    }

    true
}

fn check_safe_tolerant(levels: &[u64]) -> bool {
    if check_safe(levels) {
        return true;
    }

    for (i, _) in levels.iter().enumerate() {
        let mut levels_copy = levels.to_vec();
        levels_copy.remove(i);
        if check_safe(&levels_copy) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let levels: Vec<u64> = line
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        if check_safe(&levels) {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let levels: Vec<u64> = line
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        if check_safe_tolerant(&levels) {
            result += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
