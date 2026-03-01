mod grid;
mod tetromino;
mod ui;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

use grid::Grid;
use tetromino::Tetromino;

fn main() {
    ui::setup_terminal();

    let mut grid = Grid::new();
    let mut active_piece = Tetromino::random();
    let mut game_over = false;

    // Game speed logic - drop piece every 500ms
    let drop_interval = Duration::from_millis(500);
    let mut last_drop_time = Instant::now();

    loop {
        if game_over {
            break;
        }

        // Update UI
        ui::draw(&grid, &active_piece);

        // Input handling with timeout
        // Wait for an event until it's time to drop the piece again.
        let elapsed = last_drop_time.elapsed();
        let timeout = drop_interval
            .checked_sub(elapsed)
            .unwrap_or_else(|| Duration::from_millis(0));

        if event::poll(timeout).unwrap() {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read().unwrap()
            {
                // Exit shortcuts
                if code == KeyCode::Esc
                    || code == KeyCode::Char('q')
                    || (code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL))
                {
                    break;
                }

                match code {
                    KeyCode::Left => {
                        active_piece.move_left();
                        if grid.is_collision(&active_piece) {
                            active_piece.move_right(); // undo
                        }
                    }
                    KeyCode::Right => {
                        active_piece.move_right();
                        if grid.is_collision(&active_piece) {
                            active_piece.move_left(); // undo
                        }
                    }
                    KeyCode::Down => {
                        // Soft drop
                        active_piece.move_down();
                        if grid.is_collision(&active_piece) {
                            active_piece.y -= 1; // undo move

                            grid.lock(&active_piece);
                            grid.clear_lines();

                            active_piece = Tetromino::random();
                            if grid.is_collision(&active_piece) {
                                game_over = true;
                            }
                        }
                    }
                    KeyCode::Up => {
                        // Rotate
                        active_piece.rotate();
                        if grid.is_collision(&active_piece) {
                            // Simple wall kick: if rotation collides, try moving left or right by 1
                            active_piece.move_left();
                            if grid.is_collision(&active_piece) {
                                active_piece.move_right(); // undo move_left
                                active_piece.move_right(); // try move_right
                                if grid.is_collision(&active_piece) {
                                    active_piece.move_left(); // undo move_right
                                    active_piece.revert_rotation(); // Rotation failed
                                }
                            }
                        }
                    }
                    KeyCode::Char(' ') => {
                        // Hard drop
                        loop {
                            active_piece.move_down();
                            if grid.is_collision(&active_piece) {
                                active_piece.y -= 1;
                                break;
                            }
                        }

                        grid.lock(&active_piece);
                        grid.clear_lines();

                        active_piece = Tetromino::random();
                        if grid.is_collision(&active_piece) {
                            game_over = true;
                        }
                        // Instantly reset the natural drop timer to afford the player a fresh chance
                        last_drop_time = Instant::now();
                    }
                    _ => {}
                }
            }
        }

        // Natural drop over time
        if last_drop_time.elapsed() >= drop_interval {
            active_piece.move_down();
            if grid.is_collision(&active_piece) {
                active_piece.y -= 1; // back out of collision

                grid.lock(&active_piece);
                grid.clear_lines();

                active_piece = Tetromino::random();
                if grid.is_collision(&active_piece) {
                    game_over = true;
                }
            }
            last_drop_time = Instant::now();
        }
    }

    ui::restore_terminal();

    if game_over {
        println!("Game Over! Your terminal Tetris adventure has concluded.");
    } else {
        println!("Thanks for playing!");
    }
}
