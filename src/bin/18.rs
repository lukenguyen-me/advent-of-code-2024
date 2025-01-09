use std::collections::HashSet;

advent_of_code::solution!(18);

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
    bytes: Vec<(usize, usize)>,
}

impl Grid {
    fn new(width: usize, height: usize, input: &str) -> Self {
        let cells: Vec<Vec<char>> = vec![vec!['.'; width]; height];
        let mut bytes = vec![];

        for line in input.trim().lines() {
            let (x, y) = line.split_once(",").unwrap();
            let row: usize = y.parse().unwrap();
            let col: usize = x.parse().unwrap();
            bytes.push((col, row));
        }

        Self {
            width,
            height,
            cells,
            bytes,
        }
    }

    fn reset(&mut self) {
        self.cells = vec![vec!['.'; self.width]; self.height];
    }

    fn make_corrupt(&mut self, num_bytes: u64) {
        for i in 0..num_bytes {
            self.cells[self.bytes[i as usize].1][self.bytes[i as usize].0] = '#';
        }
    }

    fn find_shortest_path(&self) -> u64 {
        let mut step = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut stacks: Vec<(usize, usize)> = vec![(0 as usize, 0 as usize)];
        while stacks.len() > 0 {
            let mut temp: Vec<(usize, usize)> = vec![];
            while let Some((x, y)) = stacks.pop() {
                if x == self.width - 1 && y == self.height - 1 {
                    return step;
                }
                if x > 0 && self.cells[y][x - 1] == '.' && !visited.contains(&(x - 1, y)) {
                    visited.insert((x - 1, y));
                    temp.push((x - 1, y));
                }
                if y > 0 && self.cells[y - 1][x] == '.' && !visited.contains(&(x, y - 1)) {
                    visited.insert((x, y - 1));
                    temp.push((x, y - 1));
                }
                if x < self.width - 1
                    && self.cells[y][x + 1] == '.'
                    && !visited.contains(&(x + 1, y))
                {
                    visited.insert((x + 1, y));
                    temp.push((x + 1, y));
                }
                if y < self.height - 1
                    && self.cells[y + 1][x] == '.'
                    && !visited.contains(&(x, y + 1))
                {
                    visited.insert((x, y + 1));
                    temp.push((x, y + 1));
                }
            }
            stacks = temp;
            step += 1;
        }

        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = Grid::new(71, 71, input);
    grid.make_corrupt(1024);
    let result = grid.find_shortest_path();

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut grid = Grid::new(71, 71, input);
    let mut left = 0;
    let mut right = grid.bytes.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        grid.reset();
        grid.make_corrupt((mid + 1) as u64);
        let res = grid.find_shortest_path();

        if res == 0 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    Some(format!("{},{}", grid.bytes[right].0, grid.bytes[right].1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(234));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("58,19".to_string()));
    }
}
