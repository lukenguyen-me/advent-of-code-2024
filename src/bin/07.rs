advent_of_code::solution!(7);

static OPERATORS: [fn(u64, u64) -> u64; 3] = [
    |a, b| a + b,
    |a, b| a * b,
    |a, b| format!("{}{}", a, b).parse().unwrap(),
];

struct Equation {
    target: u64,
    numbers: Vec<u64>,
    operators: Vec<fn(u64, u64) -> u64>,
}

impl Equation {
    fn new(line: &str, operators: Vec<fn(u64, u64) -> u64>) -> Equation {
        let (target, numbers) = line.split_once(": ").unwrap();
        let target = target.parse().unwrap();
        let numbers = numbers
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u64>>();
        Equation {
            target,
            numbers,
            operators,
        }
    }

    fn evaluate(&self, numbers: &[u64], acc: u64) -> bool {
        if numbers.len() == 0 {
            return acc == self.target;
        }
        if acc > self.target {
            return false;
        }
        self.operators
            .iter()
            .any(|op| self.evaluate(&numbers[1..], op(acc, numbers[0])))
    }

    fn validate(&self) -> bool {
        self.evaluate(&self.numbers[1..], self.numbers[0])
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let equation = Equation::new(line, vec![OPERATORS[0], OPERATORS[1]]);
        if equation.validate() {
            result += equation.target;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let equation = Equation::new(line, OPERATORS.to_vec());
        if equation.validate() {
            result += equation.target;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
