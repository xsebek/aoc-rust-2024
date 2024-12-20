pub mod template;

pub struct Grid<'a> {
    pub rows: usize,
    pub cols: usize,
    pub data: &'a str
}

impl<'a> Grid<'a> {
    pub fn new(data: &'a str) -> Self {
        let rows = data.lines().count();
        let cols = data.lines().map(|l| l.len()).max().unwrap_or(0);
        Grid { rows, cols, data }
    }

    pub fn try_get(&self, row: isize, col: isize) -> Option<char> {
        if row >= self.rows as isize || col >= self.cols as isize || row < 0 || col < 0 {
            return None
        }
        Some(self.data.as_bytes()[row as usize * (self.cols + 1) + col as usize] as char)
    }
    
    pub fn get(&self, row: usize, col: usize) -> char {
        self.try_get(row as isize, col as isize).expect("col and row must be within range")
    }

    pub fn row(&self, row: usize) -> String {
        (0..self.cols).map(move |col| self.get(row, col)).collect()
    }

    pub fn col(&self, col: usize) -> String {
        (0..self.rows).map(move |row| self.get(row, col)).collect()
    }

    pub fn lr_diagonal(&self, row: usize, col: usize) -> String {
        (0..)
            .map(move |d| (row + d, col + d))
            .take_while(|&(r, c)| r < self.rows && c < self.cols)
            .map(|(r, c)| self.get(r, c))
            .collect()
    }

    pub fn rl_diagonal(&self, row: usize, col: usize) -> String {
        (0..)
            .map_while(move |d|
                if col >= d && row + d < self.cols {
                    Some((row + d, col - d))
                } else {
                    None
                }
            )
            .map(|(r, c)| self.get(r, c))
            .collect()
    }
}