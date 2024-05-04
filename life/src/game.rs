pub struct GameOfLife {
    grid: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl GameOfLife {
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid = vec![vec![false; cols]; rows];
        Self { grid, rows, cols }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for cell in row {
                print!("{}", if *cell { "*" } else { " " });
            }
            println!();
        }
    }
}
