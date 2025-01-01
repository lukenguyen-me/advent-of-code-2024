use std::collections::HashSet;

advent_of_code::solution!(12);

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            let mut row: Vec<char> = vec![];
            for c in line.chars() {
                row.push(c);
            }
            cells.push(row);
        }
        Grid {
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }

    fn check_edges(&self, x: usize, y: usize) -> Vec<u64> {
        let mut count: Vec<u64> = vec![0, 0, 0, 0];
        if y == 0 || self.cells[y][x] != self.cells[y - 1][x] {
            count[0] = 1;
        }
        if x + 1 == self.width || self.cells[y][x] != self.cells[y][x + 1] {
            count[1] = 1;
        }
        if y + 1 == self.height || self.cells[y][x] != self.cells[y + 1][x] {
            count[2] = 1;
        }
        if x == 0 || self.cells[y][x] != self.cells[y][x - 1] {
            count[3] = 1;
        }

        count
    }

    fn count_corner(&self, x: usize, y: usize) -> u64 {
        let edges = self.check_edges(x, y);
        let mut result: u64 = 0;
        if edges[0] + edges[1] == 2 {
            result += 1;
        }
        if edges[0] + edges[1] == 0
            && y > 0
            && x < self.width - 1
            && self.cells[y][x] != self.cells[y - 1][x + 1]
        {
            result += 1;
        }
        if edges[1] + edges[2] == 2 {
            result += 1;
        }
        if edges[1] + edges[2] == 0
            && x < self.width - 1
            && y < self.height - 1
            && self.cells[y][x] != self.cells[y + 1][x + 1]
        {
            result += 1;
        }
        if edges[2] + edges[3] == 2 {
            result += 1;
        }
        if edges[2] + edges[3] == 0
            && x > 0
            && y < self.height - 1
            && self.cells[y][x] != self.cells[y + 1][x - 1]
        {
            result += 1;
        }
        if edges[3] + edges[0] == 2 {
            result += 1;
        }
        if edges[3] + edges[0] == 0
            && x > 0
            && y > 0
            && self.cells[y][x] != self.cells[y - 1][x - 1]
        {
            result += 1;
        }
        result
    }

    fn get_region_stats(
        &self,
        x: usize,
        y: usize,
        region: char,
        visited: &mut HashSet<(usize, usize)>,
    ) -> (u64, u64) {
        visited.insert((x, y));
        if self.cells[y][x] != region {
            return (0, 0);
        }

        let mut result = (1, self.check_edges(x, y).iter().sum());

        if x > 0 && !visited.contains(&(x - 1, y)) && self.cells[y][x - 1] == region {
            let (area, perimeter) = self.get_region_stats(x - 1, y, region, visited);
            result.0 += area;
            result.1 += perimeter;
        }
        if y > 0 && !visited.contains(&(x, y - 1)) && self.cells[y - 1][x] == region {
            let (area, perimeter) = self.get_region_stats(x, y - 1, region, visited);
            result.0 += area;
            result.1 += perimeter;
        }
        if x < self.width - 1 && !visited.contains(&(x + 1, y)) && self.cells[y][x + 1] == region {
            let (area, perimeter) = self.get_region_stats(x + 1, y, region, visited);
            result.0 += area;
            result.1 += perimeter;
        }
        if y < self.height - 1 && !visited.contains(&(x, y + 1)) && self.cells[y + 1][x] == region {
            let (area, perimeter) = self.get_region_stats(x, y + 1, region, visited);
            result.0 += area;
            result.1 += perimeter;
        }

        result
    }

    fn get_region_stats_2(
        &self,
        x: usize,
        y: usize,
        region: char,
        visited: &mut HashSet<(usize, usize)>,
    ) -> (u64, u64) {
        visited.insert((x, y));
        if self.cells[y][x] != region {
            return (0, 0);
        }

        let mut result = (1, self.count_corner(x, y));

        if x > 0 && !visited.contains(&(x - 1, y)) && self.cells[y][x - 1] == region {
            let (area, sides) = self.get_region_stats_2(x - 1, y, region, visited);
            result.0 += area;
            result.1 += sides;
        }
        if y > 0 && !visited.contains(&(x, y - 1)) && self.cells[y - 1][x] == region {
            let (area, sides) = self.get_region_stats_2(x, y - 1, region, visited);
            result.0 += area;
            result.1 += sides;
        }
        if x < self.width - 1 && !visited.contains(&(x + 1, y)) && self.cells[y][x + 1] == region {
            let (area, sides) = self.get_region_stats_2(x + 1, y, region, visited);
            result.0 += area;
            result.1 += sides;
        }
        if y < self.height - 1 && !visited.contains(&(x, y + 1)) && self.cells[y + 1][x] == region {
            let (area, sides) = self.get_region_stats_2(x, y + 1, region, visited);
            result.0 += area;
            result.1 += sides;
        }

        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let mut result: u64 = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if visited.contains(&(x, y)) {
                continue;
            }
            let (area, perimeter) = grid.get_region_stats(x, y, grid.cells[y][x], &mut visited);
            result += area * perimeter;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let mut result: u64 = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if visited.contains(&(x, y)) {
                continue;
            }
            let (area, sides) = grid.get_region_stats_2(x, y, grid.cells[y][x], &mut visited);
            result += area * sides;
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
