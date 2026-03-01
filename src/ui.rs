use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{Write, stdout};

use crate::grid::{Grid, HEIGHT, WIDTH};
use crate::tetromino::{Shape, Tetromino};

pub fn setup_terminal() {
    crossterm::terminal::enable_raw_mode().unwrap();
    stdout()
        .queue(EnterAlternateScreen)
        .unwrap()
        .queue(Hide)
        .unwrap()
        .queue(Clear(ClearType::All))
        .unwrap();
    stdout().flush().unwrap();
}

pub fn restore_terminal() {
    crossterm::terminal::disable_raw_mode().unwrap();
    stdout()
        .queue(Show)
        .unwrap()
        .queue(LeaveAlternateScreen)
        .unwrap();
    stdout().flush().unwrap();
}

fn map_shape_to_color(shape: Shape) -> Color {
    match shape {
        Shape::I => Color::Cyan,
        Shape::O => Color::Yellow,
        Shape::T => Color::Magenta,
        Shape::S => Color::Green,
        Shape::Z => Color::Red,
        Shape::J => Color::Blue,
        Shape::L => Color::DarkYellow, // Close to Orange
    }
}

pub fn draw(grid: &Grid, active_piece: &Tetromino) {
    let mut stdout = stdout();

    let offset_x = 20;
    let offset_y = 5;

    // Draw borders
    for y in 0..20 {
        stdout
            .queue(MoveTo(offset_x - 2, offset_y + y as u16))
            .unwrap()
            .queue(Print("<!"))
            .unwrap()
            .queue(MoveTo(offset_x + (WIDTH * 2) as u16, offset_y + y as u16))
            .unwrap()
            .queue(Print("!>"))
            .unwrap();
    }

    // Bottom border
    stdout
        .queue(MoveTo(offset_x - 2, offset_y + 20))
        .unwrap()
        .queue(Print("<!====================!>"))
        .unwrap();

    // Draw Grid (skip hidden rows 0..3, start from 4)
    for y in 4..HEIGHT {
        for x in 0..WIDTH {
            stdout
                .queue(MoveTo(offset_x + (x * 2) as u16, offset_y + (y - 4) as u16))
                .unwrap();

            if let Some(shape) = grid.cells[y][x] {
                stdout
                    .queue(SetForegroundColor(map_shape_to_color(shape)))
                    .unwrap()
                    .queue(Print("[]"))
                    .unwrap()
                    .queue(ResetColor)
                    .unwrap();
            } else {
                stdout.queue(Print(" .")).unwrap();
            }
        }
    }

    // Draw active piece
    let color = map_shape_to_color(active_piece.shape);
    for &(hx, hy) in active_piece.get_grid_positions().iter() {
        if hy >= 4 {
            // Only visible if broken through top hidden rows!
            stdout
                .queue(MoveTo(
                    offset_x + (hx * 2) as u16,
                    offset_y + (hy - 4) as u16,
                ))
                .unwrap()
                .queue(SetForegroundColor(color))
                .unwrap()
                .queue(Print("[]"))
                .unwrap()
                .queue(ResetColor)
                .unwrap();
        }
    }

    stdout.flush().unwrap();
}
