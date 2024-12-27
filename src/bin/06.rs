use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Hash, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn get_next_position(&self, position: &(isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
        }
    }
}

struct Grid {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
    init_position: (isize, isize),
    init_direction: Direction,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut position: (isize, isize) = (0, 0);
        let mut direction: Direction = Direction::Right;
        let mut grid: Vec<Vec<char>> = vec![];

        for (y, line) in input.lines().enumerate() {
            let mut row: Vec<char> = vec![];
            for (x, c) in line.chars().enumerate() {
                row.push(c);
                match c {
                    '^' => {
                        direction = Direction::Up;
                        position = (x as isize, y as isize);
                    }
                    'v' => {
                        direction = Direction::Down;
                        position = (x as isize, y as isize);
                    }
                    '>' => {
                        direction = Direction::Right;
                        position = (x as isize, y as isize);
                    }
                    '<' => {
                        direction = Direction::Left;
                        position = (x as isize, y as isize);
                    }
                    _ => (),
                }
            }
            grid.push(row);
        }
        return Self {
            width: grid[0].len(),
            height: grid.len(),
            cells: grid,
            init_position: position,
            init_direction: direction.clone(),
        };
    }

    fn get_next_move(
        &self,
        position: &(isize, isize),
        direction: &Direction,
    ) -> ((isize, isize), Direction) {
        let mut next_position = direction.get_next_position(position);
        let mut next_direction = direction.clone();
        while self.is_valid_position(&next_position)
            && self.cells[next_position.1 as usize][next_position.0 as usize] == '#'
        {
            next_direction = next_direction.get_next();
            next_position = next_direction.get_next_position(position);
        }
        (next_position, next_direction)
    }

    fn is_valid_position(&self, position: &(isize, isize)) -> bool {
        position.0 >= 0
            && position.1 >= 0
            && position.0 < self.width as isize
            && position.1 < self.height as isize
    }

    fn move_guard(&mut self, position: &mut (isize, isize), direction: &mut Direction) {
        (*position, *direction) = self.get_next_move(position, direction);
    }

    fn set_cell(&mut self, position: (isize, isize), value: char) {
        self.cells[position.1 as usize][position.0 as usize] = value;
    }

    fn is_loop(
        &self,
        routes: &mut HashSet<((isize, isize), Direction)>,
        start_position: &(isize, isize),
        start_direction: &Direction,
    ) -> bool {
        if !self.is_valid_position(start_position) {
            return false;
        }
        let (position, direction) = self.get_next_move(start_position, start_direction);
        let new_route = (position, direction.clone());
        if routes.contains(&new_route) {
            return true;
        } else {
            routes.insert(new_route);
            let result = self.is_loop(routes, &position, &direction);
            routes.remove(&new_route);
            return result;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut positions: HashSet<(isize, isize)> = std::collections::HashSet::new();
    let mut grid = Grid::new(input);
    let mut guard_position = grid.init_position;
    let mut guard_direction = grid.init_direction.clone();

    while grid.is_valid_position(&guard_position) {
        positions.insert(guard_position);
        grid.move_guard(&mut guard_position, &mut guard_direction);
    }

    Some(positions.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new(input);
    let mut guard_position = grid.init_position;
    let mut guard_direction = grid.init_direction.clone();
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    let mut visited: HashSet<((isize, isize), Direction)> = std::collections::HashSet::new();

    while grid.is_valid_position(&guard_position) {
        let obstackle_position = grid.get_next_move(&guard_position, &guard_direction);
        if grid.is_valid_position(&obstackle_position.0)
            && !positions.contains(&obstackle_position.0)
        {
            grid.set_cell(obstackle_position.0, '#');
            if grid.is_loop(&mut visited, &guard_position, &guard_direction) {
                positions.insert(obstackle_position.0);
            }
            grid.set_cell(obstackle_position.0, '.');
        }
        grid.move_guard(&mut guard_position, &mut guard_direction);
        visited.insert((guard_position, guard_direction));
    }

    Some(positions.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
