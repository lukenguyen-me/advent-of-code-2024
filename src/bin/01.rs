advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut arr1: Vec<u64> = vec![];
    let mut arr2: Vec<u64> = vec![];

    let lines = input.lines();
    for line in lines {
        let (a, b) = line.split_once("   ").unwrap();
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        arr1.push(a);
        arr2.push(b);
    }

    arr1.sort();
    arr2.sort();

    let mut sum = 0;
    for (a, b) in arr1.iter().zip(arr2.iter()) {
        sum += a.abs_diff(*b);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut arr1: Vec<u64> = vec![];
    let mut arr2: Vec<u64> = vec![];

    let lines = input.lines();
    for line in lines {
        let (a, b) = line.split_once("   ").unwrap();
        let a: u64 = a.parse().unwrap();
        let b: u64 = b.parse().unwrap();

        arr1.push(a);
        arr2.push(b);
    }

    let mut freq = std::collections::HashMap::new();
    for &b in arr2.iter() {
        *freq.entry(b).or_insert(0) += 1;
    }

    let mut result: u64 = 0;
    for x in arr1.iter() {
        if let Some(y) = freq.get(x) {
            if *y > 0 {
                result += x * y;
            }
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
