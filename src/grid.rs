use crate::tetromino::{Shape, Tetromino};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24; // 20 visible + 4 hidden

pub struct Grid {
    pub cells: [[Option<Shape>; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[None; WIDTH]; HEIGHT],
        }
    }

    pub fn is_collision(&self, piece: &Tetromino) -> bool {
        for &(x, y) in piece.get_grid_positions().iter() {
            // Check boundaries (walls and floor)
            if x < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
                return true;
            }

            // Check existing blocks
            // We only check if y >= 0 because pieces can spawn partially off-screen
            if y >= 0 {
                if self.cells[y as usize][x as usize].is_some() {
                    return true;
                }
            }
        }
        false
    }

    pub fn lock(&mut self, piece: &Tetromino) {
        for &(x, y) in piece.get_grid_positions().iter() {
            if y >= 0 && y < HEIGHT as i32 && x >= 0 && x < WIDTH as i32 {
                self.cells[y as usize][x as usize] = Some(piece.shape);
            }
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut y = HEIGHT - 1;

        loop {
            // Check if the row is full
            let is_full = self.cells[y].iter().all(|c| c.is_some());

            if is_full {
                lines_cleared += 1;
                // Move everything above it down by 1
                for i in (1..=y).rev() {
                    self.cells[i] = self.cells[i - 1];
                }
                // Clear the absolute top line
                self.cells[0] = [None; WIDTH];
                // Do NOT decrement y, because the line that just came down might also be full.
            } else {
                if y == 0 {
                    break;
                }
                y -= 1;
            }
        }

        lines_cleared
    }
}
