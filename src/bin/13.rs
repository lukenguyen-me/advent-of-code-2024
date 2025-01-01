use std::collections::HashMap;

advent_of_code::solution!(13);

struct Machine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

impl Machine {
    fn new(input: &str) -> Self {
        let mut lines = input.trim().lines();
        let button_a_text = lines.next().unwrap();
        let button_a_re = regex::Regex::new(r"X\+(\d+),\sY\+(\d+)").unwrap();
        let button_a_captures = button_a_re.captures(button_a_text).unwrap();
        let button_a = (
            button_a_captures.get(1).unwrap().as_str().parse().unwrap(),
            button_a_captures.get(2).unwrap().as_str().parse().unwrap(),
        );

        let button_b_text = lines.next().unwrap();
        let button_b_re = regex::Regex::new(r"X\+(\d+),\sY\+(\d+)").unwrap();
        let button_b_captures = button_b_re.captures(button_b_text).unwrap();
        let button_b = (
            button_b_captures.get(1).unwrap().as_str().parse().unwrap(),
            button_b_captures.get(2).unwrap().as_str().parse().unwrap(),
        );

        let prize_text = lines.next().unwrap();
        let prize_re = regex::Regex::new(r"X\=(\d+),\sY\=(\d+)").unwrap();
        let prize_captures = prize_re.captures(prize_text).unwrap();
        let prize = (
            prize_captures.get(1).unwrap().as_str().parse().unwrap(),
            prize_captures.get(2).unwrap().as_str().parse().unwrap(),
        );
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    // 94A + 22B = 8400
    // 34A + 67B = 5400
    fn calculate_cost(&self) -> u64 {
        let y = (self.prize.0 * self.button_b.1).abs_diff(self.prize.1 * self.button_b.0);
        let x = (self.button_a.0 * self.button_b.1).abs_diff(self.button_a.1 * self.button_b.0);

        if y % x != 0 {
            return 0;
        }

        let count_a: u64 = y / x;
        let temp = self.prize.0 - count_a * self.button_a.0;
        if temp % self.button_b.0 != 0 {
            return 0;
        }
        let count_b = temp / self.button_b.0;
        count_a * 3 + count_b
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines: Vec<&str> = input.split("\n\n").collect();
    let mut result: u64 = 0;

    for text in machines {
        let machine = Machine::new(text);

        if machine.prize.0 > machine.button_a.0 * 100 + machine.button_b.0 * 100 {
            continue;
        }
        if machine.prize.1 > machine.button_a.1 * 100 + machine.button_b.1 * 100 {
            continue;
        }

        result += machine.calculate_cost();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines: Vec<&str> = input.split("\n\n").collect();
    let mut result: u64 = 0;

    for machine in machines {
        let mut machine = Machine::new(machine);
        machine.prize.0 = machine.prize.0 + 10000000000000;
        machine.prize.1 = machine.prize.1 + 10000000000000;

        result += machine.calculate_cost();
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
