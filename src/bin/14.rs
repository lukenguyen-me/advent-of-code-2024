use std::{collections::HashMap, io::Write};

advent_of_code::solution!(14);

struct Robot {
    max_x: isize,
    max_y: isize,
    velocity: (isize, isize),
    x: isize,
    y: isize,
}

impl Robot {
    fn new(input: &str, max_x: isize, max_y: isize) -> Robot {
        let position_re = regex::Regex::new(r"p=(-?\d+),(-?\d+)").unwrap();
        let position = position_re.captures(input).unwrap();
        let position: (isize, isize) = (
            position.get(1).unwrap().as_str().parse().unwrap(),
            position.get(2).unwrap().as_str().parse().unwrap(),
        );

        let velocity_re = regex::Regex::new(r"v=(-?\d+),(-?\d+)").unwrap();
        let velocity = velocity_re.captures(input).unwrap();
        let velocity: (isize, isize) = (
            velocity.get(1).unwrap().as_str().parse().unwrap(),
            velocity.get(2).unwrap().as_str().parse().unwrap(),
        );

        Robot {
            max_x: max_x,
            max_y: max_y,
            velocity: velocity,
            x: position.0,
            y: position.1,
        }
    }

    fn move_on_map(&mut self, seconds: isize) {
        self.x = (self.x + self.velocity.0 * seconds) % self.max_x;
        self.y = (self.y + self.velocity.1 * seconds) % self.max_y;

        if self.x < 0 {
            self.x += self.max_x;
        }
        if self.y < 0 {
            self.y += self.max_y;
        }
    }

    fn get_quadrant(&self) -> usize {
        let w_half = self.max_x / 2;
        let h_half = self.max_y / 2;
        if self.x < w_half && self.y < h_half {
            1
        } else if self.x >= w_half + 1 && self.y < h_half {
            2
        } else if self.x >= w_half + 1 && self.y >= h_half + 1 {
            3
        } else if self.x < w_half && self.y >= h_half + 1 {
            4
        } else {
            0
        }
    }
}

fn check_straight_line(map: &HashMap<(usize, usize), usize>, width: usize, height: usize) -> bool {
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if map.contains_key(&(x, y)) {
                count += 1;
                if count >= 10 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }
    for x in 0..width {
        for y in 0..height {
            if map.contains_key(&(x, y)) {
                count += 1;
                if count >= 10 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }
    false
}

fn print_visual(
    map: &HashMap<(usize, usize), usize>,
    width: usize,
    height: usize,
    seconds: isize,
    file: &mut std::fs::File,
) {
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            if map.contains_key(&(x, y)) {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    file.write_all(format!("\nSeconds: {}\n", seconds).as_bytes())
        .unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

pub fn part_one(input: &str) -> Option<u64> {
    // let width: usize = 101;
    // let height: usize = 103;
    let width: usize = 11;
    let height: usize = 7;
    let mut scores: HashMap<usize, u64> = HashMap::new();
    scores.insert(1, 0);
    scores.insert(2, 0);
    scores.insert(3, 0);
    scores.insert(4, 0);

    for line in input.lines() {
        let mut robot = Robot::new(line, width as isize, height as isize);
        robot.move_on_map(100);
        let quadrant = robot.get_quadrant();
        *scores.entry(quadrant).or_insert(1) += 1;
    }

    Some(scores[&1] * scores[&2] * scores[&3] * scores[&4])
}

pub fn part_two(input: &str) -> Option<u64> {
    let width: usize = 101;
    let height: usize = 103;
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        robots.push(Robot::new(line, width as isize, height as isize));
    }

    let mut file = std::fs::File::create("output.txt").unwrap();

    for i in 1..=10000 {
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();
        for robot in &mut robots {
            robot.move_on_map(1);
            *map.entry((robot.x as usize, robot.y as usize)).or_insert(0) += 1;
        }

        let is_straight = check_straight_line(&map, width, height);
        if is_straight {
            print_visual(&map, width, height, i, &mut file);
        }
        println!("Seconds: {} {}", i, is_straight);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
