use std::collections::HashSet;

advent_of_code::solution!(10);

struct Grid {
    cells: Vec<Vec<u8>>,
    heads: Vec<(usize, usize)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells: Vec<Vec<u8>> = vec![];
        let mut heads: Vec<(usize, usize)> = vec![];
        for (i, line) in input.lines().enumerate() {
            let mut row: Vec<u8> = vec![];
            for (j, c) in line.chars().enumerate() {
                row.push(c as u8 - '0' as u8);
                if c == '0' {
                    heads.push((j, i));
                }
            }
            cells.push(row);
        }
        Grid { cells, heads }
    }

    fn find_last(&self, x: usize, y: usize, height: u8, tails: &mut HashSet<(usize, usize)>) {
        if x >= self.cells[0].len() || y >= self.cells.len() {
            return;
        }
        if self.cells[y][x] != height {
            return;
        }
        if self.cells[y][x] == 9 {
            tails.insert((x, y));
            return;
        }
        self.find_last(x + 1, y, height + 1, tails);
        self.find_last(x, y + 1, height + 1, tails);
        if x > 0 {
            self.find_last(x - 1, y, height + 1, tails)
        }
        if y > 0 {
            self.find_last(x, y - 1, height + 1, tails)
        }
    }

    fn count_path(&self, x: usize, y: usize, height: u8) -> u64 {
        if x >= self.cells[0].len() || y >= self.cells.len() {
            return 0;
        }
        if self.cells[y][x] != height {
            return 0;
        }
        if self.cells[y][x] == 9 {
            return 1;
        }
        let mut sum: u64 = 0;
        sum += self.count_path(x + 1, y, height + 1);
        sum += self.count_path(x, y + 1, height + 1);
        if x > 0 {
            sum += self.count_path(x - 1, y, height + 1);
        }
        if y > 0 {
            sum += self.count_path(x, y - 1, height + 1);
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Grid::new(input);
    let mut result: u64 = 0;
    for head in map.heads.clone() {
        let mut tails: HashSet<(usize, usize)> = HashSet::new();
        map.find_last(head.0, head.1, 0, &mut tails);
        result += tails.len() as u64;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Grid::new(input);
    let mut result: u64 = 0;
    for head in map.heads.clone() {
        result += map.count_path(head.0, head.1, 0);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
