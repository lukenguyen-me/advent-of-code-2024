advent_of_code::solution!(17);

type Num = u64;

struct Computer {
    register_a: Num,
    register_b: Num,
    register_c: Num,
    program: Vec<Num>,
    instruction_pointer: usize,
    outputs: Vec<Num>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut lines = input.trim().lines();
        let register_a_text = lines.next().unwrap();
        let a = register_a_text.split_once(": ").unwrap().1;
        let register_b_text = lines.next().unwrap();
        let b = register_b_text.split_once(": ").unwrap().1;
        let register_c_text = lines.next().unwrap();
        let c = register_c_text.split_once(": ").unwrap().1;

        lines.next();
        let instructions_text = lines.next().unwrap();
        let instructions_text = instructions_text.split_once(": ").unwrap().1;
        let program: Vec<Num> = instructions_text
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            register_a: a.parse().unwrap(),
            register_b: b.parse().unwrap(),
            register_c: c.parse().unwrap(),
            instruction_pointer: 0,
            program: program,
            outputs: Vec::new(),
        }
    }

    fn get_combo_operand(&self, input: Num) -> Num {
        match input {
            0..=3 => input,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("combo operand greater than 6"),
        }
    }

    fn execute(&mut self) {
        let instruction = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];
        match instruction {
            0 => {
                let operand = self.get_combo_operand(operand);
                self.register_a >>= operand;
                self.instruction_pointer += 2;
            }
            1 => {
                self.register_b ^= operand;
                self.instruction_pointer += 2;
            }
            2 => {
                let operand = self.get_combo_operand(operand);
                self.register_b = operand & 7;
                self.instruction_pointer += 2;
            }
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand.try_into().unwrap();
                } else {
                    self.instruction_pointer += 2;
                }
            }
            4 => {
                self.register_b ^= self.register_c;
                self.instruction_pointer += 2;
            }
            5 => {
                let operand = self.get_combo_operand(operand);
                self.outputs.push(operand & 7);
                self.instruction_pointer += 2;
            }
            6 => {
                let operand = self.get_combo_operand(operand);
                self.register_b = self.register_a >> operand;
                self.instruction_pointer += 2;
            }
            7 => {
                let operand = self.get_combo_operand(operand);
                self.register_c = self.register_a >> operand;
                self.instruction_pointer += 2;
            }
            _ => panic!("opcode greater than 7"),
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            self.execute()
        }
    }

    fn run_program(&mut self, a: Num) -> Vec<Num> {
        self.register_a = a;
        self.instruction_pointer = 0;
        self.outputs.clear();
        self.run();
        return self.outputs.clone();
    }

    fn print_output(&self) -> String {
        self.outputs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn dfs(computer: &mut Computer, position: isize, curr: Num) -> Option<Num> {
    if position < 0 {
        return Some(curr);
    }
    let new_curr = curr << 3;
    let target = computer.program[position as usize];
    for i in 0..8 {
        let n = i + new_curr;
        let digit = computer.run_program(n)[0];
        if digit == target {
            let out = dfs(computer, position - 1, n);
            if let Some(res) = out {
                return Some(res);
            }
        }
    }
    return None;
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::new(input);
    computer.run();

    Some(computer.print_output())
}

pub fn part_two(input: &str) -> Option<Num> {
    let mut computer = Computer::new(input);
    let position = computer.program.len() - 1;

    let result = dfs(&mut computer, position as isize, 0);

    if let Some(result) = result {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
