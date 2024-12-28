use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

advent_of_code::solution!(8);

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
    antenna: HashMap<char, Vec<(usize, usize)>>,
}

impl Grid {
    fn from(input: &str) -> Self {
        let mut cells: Vec<Vec<char>> = vec![];
        let mut antenna: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            let mut row: Vec<char> = vec![];
            for (x, c) in line.chars().enumerate() {
                row.push(c);
                if c != '.' {
                    antenna.entry(c).or_insert(vec![]).push((x, y));
                }
            }
            cells.push(row);
        }
        Grid {
            width: cells[0].len(),
            height: cells.len(),
            cells,
            antenna,
        }
    }

    fn valid_point(&self, point: (usize, usize)) -> bool {
        point.0 < self.width && point.1 < self.height
    }

    fn get_antinode(
        &self,
        from_point: (usize, usize),
        to_point: (usize, usize),
    ) -> Option<(usize, usize)> {
        if to_point.0 * 2 < from_point.0 || to_point.1 * 2 < from_point.1 {
            return None;
        }
        let x = to_point.0 * 2 - from_point.0;
        let y = to_point.1 * 2 - from_point.1;
        if !self.valid_point((x, y)) {
            return None;
        }
        Some((x, y))
    }

    fn detect_antinode_p2(
        &self,
        antinodes: &mut HashSet<(usize, usize)>,
        point_1: (usize, usize),
        point_2: (usize, usize),
    ) {
        let mut prev = point_1;
        let mut next = point_2;
        loop {
            if let Some(antinode) = self.get_antinode(prev, next) {
                antinodes.insert(antinode);
                prev = next;
                next = antinode;
            } else {
                break;
            }
        }

        prev = point_2;
        next = point_1;
        loop {
            if let Some(antinode) = self.get_antinode(prev, next) {
                antinodes.insert(antinode);
                prev = next;
                next = antinode;
            } else {
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from(input);
    let mut antnodes = HashSet::<(usize, usize)>::new();

    for antennas in grid.antenna.values() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                if let Some(antinode) = grid.get_antinode(antennas[i], antennas[j]) {
                    antnodes.insert(antinode);
                }
                if let Some(antinode) = grid.get_antinode(antennas[j], antennas[i]) {
                    antnodes.insert(antinode);
                }
            }
        }
    }
    Some(antnodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from(input);
    let mut antinodes = HashSet::<(usize, usize)>::new();

    for antennas in grid.antenna.values() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                antinodes.insert(antennas[i]);
                antinodes.insert(antennas[j]);
                grid.detect_antinode_p2(&mut antinodes, antennas[i], antennas[j]);
            }
        }
    }

    Some(antinodes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
