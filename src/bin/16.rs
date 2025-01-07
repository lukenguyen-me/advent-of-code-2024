use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::Add,
};

advent_of_code::solution!(16);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn get_axis(&self) -> usize {
        match self {
            Direction::Up | Direction::Down => 0,
            Direction::Right | Direction::Left => 1,
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (col, row) = self;
        match rhs {
            Direction::Up => (col, row - 1),
            Direction::Right => (col + 1, row),
            Direction::Down => (col, row + 1),
            Direction::Left => (col - 1, row),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    x: usize,
    y: usize,
    direction: Direction,
    cost: u64,
    history: Option<Vec<(usize, usize)>>,
}

struct Grid {
    cells: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells: Vec<Vec<char>> = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut row: Vec<char> = vec![];
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (x, y);
                }
                if c == 'E' {
                    end = (x, y);
                }
                row.push(c);
            }
            cells.push(row);
        }

        Self {
            start,
            end,
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }

    fn dijkstra(&self) -> u64 {
        let mut min_cost = u64::MAX;
        let mut dist = vec![vec![u64::MAX; self.width]; self.height];
        let mut prio = BinaryHeap::new();

        dist[self.start.1][self.start.0] = 0;
        prio.push(Reverse(Tile {
            x: self.start.0,
            y: self.start.1,
            direction: Direction::Right,
            cost: 0,
            history: None,
        }));

        while let Some(Reverse(Tile {
            x,
            y,
            direction,
            cost,
            history,
        })) = prio.pop()
        {
            if x == self.end.0 && y == self.end.1 && cost < min_cost {
                min_cost = cost;
                continue;
            }
            if cost > dist[y][x] || cost >= min_cost {
                continue;
            }
            for dir in Direction::ALL {
                let next_dir = dir;
                let (next_x, next_y) = (x, y) + next_dir;
                let mut next_cost = cost;
                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_dir.rotate();
                    next_cost += 1001;
                }
                if self.cells[next_y][next_x] == '#' {
                    continue;
                }
                if next_cost < dist[next_y][next_x] {
                    dist[next_y][next_x] = next_cost;
                    prio.push(Reverse(Tile {
                        x: next_x,
                        y: next_y,
                        direction: next_dir,
                        cost: next_cost,
                        history: None,
                    }));
                }
            }
        }

        min_cost
    }

    fn dijkstra_backtrack(&self, min_cost: u64) -> u64 {
        let mut to_visit = vec![vec![[min_cost, min_cost]; self.width]; self.height];
        let mut prio = BinaryHeap::new();
        let mut tiles: HashSet<(usize, usize)> = HashSet::new();

        to_visit[self.start.1][self.start.0][Direction::get_axis(&Direction::Right)] = 0;

        prio.push(Reverse(Tile {
            x: self.start.0,
            y: self.start.1,
            direction: Direction::Right,
            cost: 0,
            history: Some(vec![]),
        }));
        while let Some(Reverse(Tile {
            x,
            y,
            direction,
            cost,
            history,
        })) = prio.pop()
        {
            let mut history = history.unwrap();
            history.push((x, y));

            if cost > to_visit[y][x][direction.get_axis()] || cost > min_cost {
                continue;
            }
            if x == self.end.0 && y == self.end.1 {
                if cost == min_cost {
                    tiles.extend(history);
                }
                continue;
            }
            for dir in Direction::ALL {
                let next_dir = dir;
                let (next_x, next_y) = (x, y) + next_dir;
                let mut next_cost = cost;
                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_dir.rotate();
                    next_cost += 1001;
                }
                if self.cells[next_y][next_x] == '#' {
                    continue;
                }
                if next_cost <= to_visit[next_y][next_x][direction.get_axis()] {
                    to_visit[next_y][next_x][direction.get_axis()] = next_cost;
                    prio.push(Reverse(Tile {
                        x: next_x,
                        y: next_y,
                        direction: next_dir,
                        cost: next_cost,
                        history: Some(history.clone()),
                    }));
                }
            }
        }

        tiles.len() as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);

    Some(grid.dijkstra())
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let min_cost = grid.dijkstra();

    Some(grid.dijkstra_backtrack(min_cost))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
