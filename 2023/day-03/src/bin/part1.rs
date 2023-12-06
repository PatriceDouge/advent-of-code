fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(&input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let mut adjacent_num: Vec<u32> = Vec::new();
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().filter(|&c| c != ' ').collect())
        .collect();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] != '.' && !is_symbol(grid[row][col]) && is_symbol_adjacent(&grid, row, col) {
                if let Some(_digit) = grid[row][col].to_digit(10) {
                    adjacent_num.push(extract_number(&mut grid, row, col));
                }
            } else {
                continue;
            }
        }
    }
    
    return adjacent_num.iter().sum::<u32>();
}

fn is_symbol_adjacent(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut adjacent = false;

    if row > 0 && is_symbol( grid[row - 1][col]) {
        adjacent = true;
    } else if row > 0 && col > 0 && is_symbol( grid[row - 1][col - 1]) {
        adjacent = true;
    } else if row > 0 && col < grid[row].len() - 1 && is_symbol( grid[row - 1][col + 1]) {
        adjacent = true;
    } else if row < grid.len() - 1 && is_symbol(grid[row + 1][col]) {
        adjacent = true;
    } else if row < grid.len() - 1 && col > 0 && is_symbol(grid[row + 1][col - 1]) {
        adjacent = true;
    } else if row < grid.len() - 1 && col <  grid[row].len() -1 && is_symbol(grid[row + 1][col + 1]) {
        adjacent = true;
    } else if col > 0 && is_symbol(grid[row][col - 1]) {
        adjacent = true;
    } else if col < grid[row].len() - 1 && is_symbol(grid[row][col + 1]) {
        adjacent = true;
    }

    return adjacent;
}

fn is_symbol(char : char) -> bool {
    return !char.is_digit(10) && char != '.';
}

fn extract_number(grid: &mut Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut num_str = String::new();
    let mut start_col = col;

    while start_col > 0 && grid[row][start_col - 1].is_digit(10) {
        start_col -= 1;
    }

    while start_col < grid[row].len() && grid[row][start_col].is_digit(10) {
        num_str.push(grid[row][start_col]);
        grid[row][start_col] = ' ';
        start_col += 1;
    }
    return num_str.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        assert_eq!(4361, process(input));
        Ok(())
    }
}