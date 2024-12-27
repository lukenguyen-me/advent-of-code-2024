advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    for cap in re.captures_iter(input) {
        let x = cap[1].parse::<u64>().unwrap();
        let y = cap[2].parse::<u64>().unwrap();
        result += x * y;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = regex::Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))").unwrap();
    let mut result = 0;
    let mut enable = true;
    for cap in re.captures_iter(input) {
        let command = cap[0].to_string();
        if command == "don't()" {
            enable = false;
        } else if command == "do()" {
            enable = true;
        } else if enable {
            let x = cap[1].parse::<u64>().unwrap();
            let y = cap[2].parse::<u64>().unwrap();
            result += x * y;
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
