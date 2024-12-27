advent_of_code::solution!(4);

// function to check if words are the same or reversed each other, otherwise return false
fn check_words(words: Vec<char>, target: &str) -> bool {
    let target = target.chars().collect::<Vec<char>>();
    let reverse_words = words.clone().into_iter().rev().collect::<Vec<char>>();
    words == target || reverse_words == target
}

fn scan_cell(grid: &Vec<Vec<char>>, x: usize, y: usize, m: usize, n: usize) -> u64 {
    let mut result: u64 = 0;

    // horizontal
    if x + 3 < n
        && check_words(
            vec![grid[y][x], grid[y][x + 1], grid[y][x + 2], grid[y][x + 3]],
            "XMAS",
        )
    {
        result += 1;
    }

    // vertical
    if y + 3 < m
        && check_words(
            vec![grid[y][x], grid[y + 1][x], grid[y + 2][x], grid[y + 3][x]],
            "XMAS",
        )
    {
        result += 1;
    }

    // diagonal
    if x + 3 < n
        && y + 3 < m
        && check_words(
            vec![
                grid[y][x],
                grid[y + 1][x + 1],
                grid[y + 2][x + 2],
                grid[y + 3][x + 3],
            ],
            "XMAS",
        )
    {
        result += 1;
    }

    // anti-diagonal
    if x + 3 < n
        && y + 3 < m
        && check_words(
            vec![
                grid[y + 3][x],
                grid[y + 2][x + 1],
                grid[y + 1][x + 2],
                grid[y][x + 3],
            ],
            "XMAS",
        )
    {
        result += 1;
    }

    result
}

fn scan_cell_2(grid: &Vec<Vec<char>>, x: usize, y: usize, m: usize, n: usize) -> u64 {
    let mut result: u64 = 0;

    // diagonal
    if x + 2 < n && y + 2 < m {
        if check_words(
            vec![grid[y][x], grid[y + 1][x + 1], grid[y + 2][x + 2]],
            "MAS",
        ) && check_words(
            vec![grid[y + 2][x], grid[y + 1][x + 1], grid[y][x + 2]],
            "MAS",
        ) {
            result += 1;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut result: u64 = 0;

    for i in 0..m {
        for j in 0..n {
            result += scan_cell(&grid, j, i, m, n);
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut result: u64 = 0;

    for i in 0..m {
        for j in 0..n {
            result += scan_cell_2(&grid, j, i, m, n);
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
