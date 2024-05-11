use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;

pub struct GameOfLife {
    grid: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl GameOfLife {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = vec![vec![false; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if rand::random() {
                    grid[i][j] = true;
                }
            }
        }
        Self { grid, rows, cols }
    }

    pub fn print(&self) {
        stdout().execute(Clear(ClearType::All)).unwrap();
        stdout().execute(MoveTo(0, 0)).unwrap();
        for row in &self.grid {
            for cell in row {
                print!("{}", if *cell { "*" } else { " " });
            }
            println!();
        }
    }

    pub fn generate_next_states(&mut self) {
        let mut next_state = vec![vec![false; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                next_state[i][j] = self.get_next_state(i, j);
            }
        }

        self.grid = next_state;
    }

    fn get_next_state(&self, row: usize, col: usize) -> bool {
        let live_neighbours = self.count_live_neighbours(row, col);
        let is_alive = self.grid[row][col];

        match (is_alive, live_neighbours) {
            (true, 2) | (true, 3) | (false, 3) => true,
            _ => false,
        }
    }

    fn count_live_neighbours(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let mut new_row = row as i32 + i;
                let mut new_col = col as i32 + j;

                if new_row < 0 {
                    new_row = self.rows as i32 - 1;
                } else if new_row >= self.rows as i32 {
                    new_row = 0;
                }

                if new_col < 0 {
                    new_col = self.cols as i32 - 1;
                } else if new_col >= self.cols as i32 {
                    new_col = 0;
                }

                if self.grid[new_row as usize][new_col as usize] {
                    count += 1;
                }
            }
        }

        count
    }
}
