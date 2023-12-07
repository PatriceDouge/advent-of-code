fn main() {
    let input = include_str!("../../input2.txt");
    let output = process(&input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let mut gear_ratio_sum: Vec<u32> = Vec::new();
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().filter(|&c| c != ' ').collect())
        .collect();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '*' {
                let gear_ratio = is_num_adjacent(&mut grid, row, col);
                if gear_ratio > 0 {
                    gear_ratio_sum.push(gear_ratio);
                }
            } else {
                continue;
            }
        }
    }
    
    return gear_ratio_sum.iter().sum::<u32>();
}

fn is_num_adjacent(grid: &mut Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut gear_ratio: Vec<u32> = Vec::new();

    if row > 0 && grid[row - 1][col].is_digit(10) {
        let num = extract_number(grid, row - 1, col);
        gear_ratio.push(num);
    } if row > 0 && col > 0 && grid[row - 1][col - 1].is_digit(10) {
        let num = extract_number(grid, row - 1, col - 1);
        gear_ratio.push(num);
    } if row > 0 && col < grid[row].len() - 1 && grid[row - 1][col + 1].is_digit(10) {
        let num = extract_number(grid, row - 1, col + 1);
        gear_ratio.push(num);
    } if row < grid.len() - 1 && grid[row + 1][col].is_digit(10) {
        let num = extract_number(grid, row + 1, col);
        gear_ratio.push(num);
    } if row < grid.len() - 1 && col > 0 && grid[row + 1][col - 1].is_digit(10) {
        let num = extract_number(grid, row + 1, col - 1);
        gear_ratio.push(num);
    } if row < grid.len() - 1 && col <  grid[row].len() - 1 && grid[row + 1][col + 1].is_digit(10) {
        let num = extract_number(grid, row + 1, col + 1);
        gear_ratio.push(num);
    } if col > 0 && grid[row][col - 1].is_digit(10) {
        let num = extract_number(grid, row, col - 1);
        gear_ratio.push(num);
    } if col < grid[row].len() - 1 && grid[row][col + 1].is_digit(10) {
        let num = extract_number(grid, row, col + 1);
        gear_ratio.push(num);
    }

    dbg!(&gear_ratio);

    if gear_ratio.len() == 2 {
        return gear_ratio[0] * gear_ratio[1];
    } else {
        return 0;
    }
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
    return num_str.parse::<u32>().unwrap_or(0);
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
        assert_eq!(467835, process(input));
        Ok(())
    }
}