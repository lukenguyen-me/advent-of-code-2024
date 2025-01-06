advent_of_code::solution!(15);

enum Move {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl Grid {
    fn new(input: &str, scale: bool) -> Grid {
        let mut cells: Vec<Vec<char>> = Vec::new();
        let mut robot = (0, 0);
        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                if scale {
                    match c {
                        '@' => {
                            row.push('@');
                            robot = (row.len() - 1, cells.len());
                            row.push('.');
                        }
                        'O' => {
                            row.push('[');
                            row.push(']');
                        }
                        _ => {
                            row.push(c);
                            row.push(c);
                        }
                    }
                } else {
                    row.push(c);
                    if c == '@' {
                        robot = (row.len() - 1, cells.len());
                    }
                }
            }
            cells.push(row);
        }

        Grid {
            width: cells[0].len(),
            height: cells.len(),
            cells: cells,
            robot: robot,
        }
    }

    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.cells[i][j]);
            }
            println!();
        }
    }

    fn check_can_move(&self, cell: (usize, usize), direction: &Move) -> bool {
        if self.cells[cell.1][cell.0] == '.' {
            return true;
        }
        if self.cells[cell.1][cell.0] == '#' {
            return false;
        }
        if self.cells[cell.1][cell.0] == 'O' || self.cells[cell.1][cell.0] == '@' {
            return match direction {
                Move::Up => self.check_can_move((cell.0, cell.1 - 1), direction),
                Move::Down => self.check_can_move((cell.0, cell.1 + 1), direction),
                Move::Left => self.check_can_move((cell.0 - 1, cell.1), direction),
                Move::Right => self.check_can_move((cell.0 + 1, cell.1), direction),
                Move::Unknown => false,
            };
        }
        if self.cells[cell.1][cell.0] == '[' {
            return match direction {
                Move::Up => {
                    self.check_can_move((cell.0, cell.1 - 1), direction)
                        && self.check_can_move((cell.0 + 1, cell.1 - 1), direction)
                }
                Move::Down => {
                    self.check_can_move((cell.0, cell.1 + 1), direction)
                        && self.check_can_move((cell.0 + 1, cell.1 + 1), direction)
                }
                Move::Left => self.check_can_move((cell.0 - 1, cell.1), direction),
                Move::Right => self.check_can_move((cell.0 + 2, cell.1), direction),
                Move::Unknown => false,
            };
        }
        if self.cells[cell.1][cell.0] == ']' {
            return match direction {
                Move::Up => {
                    self.check_can_move((cell.0, cell.1 - 1), direction)
                        && self.check_can_move((cell.0 - 1, cell.1 - 1), direction)
                }
                Move::Down => {
                    self.check_can_move((cell.0, cell.1 + 1), direction)
                        && self.check_can_move((cell.0 - 1, cell.1 + 1), direction)
                }
                Move::Left => self.check_can_move((cell.0 - 2, cell.1), direction),
                Move::Right => self.check_can_move((cell.0 + 1, cell.1), direction),
                Move::Unknown => false,
            };
        }
        false
    }

    /// Moves a cell in the grid based on the specified direction.
    ///
    /// # Arguments
    ///
    /// * `cell` - A tuple representing the current position of the cell (x, y).
    /// * `direction` - The direction to move the cell. Can be Up, Down, Left, Right, or Unknown.
    ///
    /// The function first checks if the cell can be moved in the given direction using
    /// `check_can_move`. If the move is possible, it performs the movement by updating
    /// the grid cells accordingly. Special handling is performed for cells containing
    /// 'O', '@', '[', and ']' to ensure correct movement and grid state.
    ///
    /// The robot's position is updated if the cell being moved is the robot itself.
    fn move_cell(&mut self, cell: (usize, usize), direction: &Move) {
        if self.cells[cell.1][cell.0] == '#' {
            return;
        }
        if self.cells[cell.1][cell.0] == '.' {
            return;
        }
        let can_move = self.check_can_move(cell, direction);
        if !can_move {
            return;
        }
        if self.cells[cell.1][cell.0] == 'O' || self.cells[cell.1][cell.0] == '@' {
            let next_cell = match direction {
                Move::Up => (cell.0, cell.1 - 1),
                Move::Down => (cell.0, cell.1 + 1),
                Move::Left => (cell.0 - 1, cell.1),
                Move::Right => (cell.0 + 1, cell.1),
                Move::Unknown => (0, 0),
            };
            self.move_cell(next_cell, direction);
            self.cells[next_cell.1][next_cell.0] = self.cells[cell.1][cell.0];
            self.cells[cell.1][cell.0] = '.';
            if self.cells[next_cell.1][next_cell.0] == '@' {
                self.robot = next_cell;
            }
        } else if self.cells[cell.1][cell.0] == '[' {
            match direction {
                Move::Up => {
                    self.move_cell((cell.0, cell.1 - 1), direction);
                    self.move_cell((cell.0 + 1, cell.1 - 1), direction);
                    self.cells[cell.1 - 1][cell.0] = '[';
                    self.cells[cell.1 - 1][cell.0 + 1] = ']';
                    self.cells[cell.1][cell.0] = '.';
                    self.cells[cell.1][cell.0 + 1] = '.';
                }
                Move::Down => {
                    self.move_cell((cell.0, cell.1 + 1), direction);
                    self.move_cell((cell.0 + 1, cell.1 + 1), direction);
                    self.cells[cell.1 + 1][cell.0] = '[';
                    self.cells[cell.1 + 1][cell.0 + 1] = ']';
                    self.cells[cell.1][cell.0] = '.';
                    self.cells[cell.1][cell.0 + 1] = '.';
                }
                Move::Left => {
                    self.move_cell((cell.0 - 1, cell.1), direction);
                    self.cells[cell.1][cell.0 - 1] = '[';
                    self.cells[cell.1][cell.0] = ']';
                    self.cells[cell.1][cell.0 + 1] = '.';
                }
                Move::Right => {
                    self.move_cell((cell.0 + 2, cell.1), direction);
                    self.cells[cell.1][cell.0 + 2] = ']';
                    self.cells[cell.1][cell.0 + 1] = '[';
                    self.cells[cell.1][cell.0] = '.';
                }
                Move::Unknown => {}
            }
        } else if self.cells[cell.1][cell.0] == ']' {
            match direction {
                Move::Up => {
                    self.move_cell((cell.0 - 1, cell.1 - 1), direction);
                    self.move_cell((cell.0, cell.1 - 1), direction);
                    self.cells[cell.1 - 1][cell.0 - 1] = '[';
                    self.cells[cell.1 - 1][cell.0] = ']';
                    self.cells[cell.1][cell.0 - 1] = '.';
                    self.cells[cell.1][cell.0] = '.';
                }
                Move::Down => {
                    self.move_cell((cell.0 - 1, cell.1 + 1), direction);
                    self.move_cell((cell.0, cell.1 + 1), direction);
                    self.cells[cell.1 + 1][cell.0 - 1] = '[';
                    self.cells[cell.1 + 1][cell.0] = ']';
                    self.cells[cell.1][cell.0 - 1] = '.';
                    self.cells[cell.1][cell.0] = '.';
                }
                Move::Left => {
                    self.move_cell((cell.0 - 2, cell.1), direction);
                    self.cells[cell.1][cell.0 - 2] = '[';
                    self.cells[cell.1][cell.0 - 1] = ']';
                    self.cells[cell.1][cell.0] = '.';
                }
                Move::Right => {
                    self.move_cell((cell.0 + 1, cell.1), direction);
                    self.cells[cell.1][cell.0 + 1] = ']';
                    self.cells[cell.1][cell.0] = '[';
                    self.cells[cell.1][cell.0 - 1] = '.';
                }
                _ => {}
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, moves) = input.trim().split_once("\n\n").unwrap();

    let mut grid = Grid::new(grid, false);
    let moves: Vec<Move> = moves
        .chars()
        .map(|c| match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => Move::Unknown,
        })
        .collect();

    for direction in moves {
        grid.move_cell(grid.robot, &direction);
    }

    let mut sum = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            if grid.cells[i][j] == 'O' {
                sum += i * 100 + j;
            }
        }
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, moves) = input.trim().split_once("\n\n").unwrap();

    let mut grid = Grid::new(grid, true);
    let moves: Vec<Move> = moves
        .chars()
        .map(|c| match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => Move::Unknown,
        })
        .collect();
    for direction in moves {
        grid.move_cell(grid.robot, &direction);
    }

    let mut sum = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            if grid.cells[i][j] == '[' {
                sum += i * 100 + j;
            }
        }
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
