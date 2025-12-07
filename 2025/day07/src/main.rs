use std::io;

fn solve(grid: &mut [Vec<char>]) -> (i32, u64) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut res = 0;
    let mut dp = vec![vec![0u64; cols]; rows];
    dp[0][grid[0].iter().position(|e| *e == 'S').unwrap()] = 1;
    for row in 1..rows {
        for col in 0..cols {
            match grid[row][col] {
                '^' if "S|".contains(grid[row - 1][col]) => {
                    res += 1;
                    if col >= 1 {
                        grid[row][col - 1] = '|';
                        dp[row][col - 1] += dp[row - 1][col];
                    }
                    if col < cols - 1 {
                        grid[row][col + 1] = '|';
                        dp[row][col + 1] += dp[row - 1][col];
                    }
                }
                '.' => {
                    dp[row][col] += dp[row - 1][col];
                    if "|S".contains(grid[row - 1][col]) {
                        grid[row][col] = '|';
                    }
                }
                _ => {
                    dp[row][col] += dp[row - 1][col];
                }
            }
        }
    }
    (res, dp.last().unwrap().iter().sum())
}

fn main() {
    let mut grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .take_while(|line: &Vec<char>| !line.is_empty())
        .collect();
    println!("{:?}", solve(&mut grid));
}
