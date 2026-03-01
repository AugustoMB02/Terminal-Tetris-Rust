use rand::RngExt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Shape {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..7) {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::S,
            4 => Shape::Z,
            5 => Shape::J,
            _ => Shape::L,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    pub shape: Shape,
    pub rotation: usize,
    pub x: i32,
    pub y: i32,
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            rotation: 0,
            x: 3, // Start near the middle of a 10-wide grid
            y: 0, // Start at the very top (hidden rows)
        }
    }

    pub fn random() -> Self {
        Self::new(Shape::random())
    }

    /// Returns the local coordinates of the blocks for the current shape and rotation.
    pub fn blocks(&self) -> [(i32, i32); 4] {
        // Shapes are defined in a 4x4 or 3x3 bounding box.
        match self.shape {
            Shape::I => {
                // I pieces pivot around the center.
                // Rotations:
                // 0: Horizontal (row 1)
                // 1: Vertical (col 2)
                // 2: Horizontal (row 2)
                // 3: Vertical (col 1)
                match self.rotation % 4 {
                    0 => [(0, 1), (1, 1), (2, 1), (3, 1)],
                    1 => [(2, 0), (2, 1), (2, 2), (2, 3)],
                    2 => [(0, 2), (1, 2), (2, 2), (3, 2)],
                    _ => [(1, 0), (1, 1), (1, 2), (1, 3)],
                }
            }
            Shape::O => {
                // O piece doesn't change relative blocks on rotation.
                [(1, 0), (2, 0), (1, 1), (2, 1)]
            }
            Shape::T => match self.rotation % 4 {
                0 => [(1, 0), (0, 1), (1, 1), (2, 1)],
                1 => [(1, 0), (1, 1), (2, 1), (1, 2)],
                2 => [(0, 1), (1, 1), (2, 1), (1, 2)],
                _ => [(1, 0), (0, 1), (1, 1), (1, 2)],
            },
            Shape::S => match self.rotation % 4 {
                0 => [(1, 0), (2, 0), (0, 1), (1, 1)],
                1 => [(1, 0), (1, 1), (2, 1), (2, 2)],
                2 => [(1, 1), (2, 1), (0, 2), (1, 2)],
                _ => [(0, 0), (0, 1), (1, 1), (1, 2)],
            },
            Shape::Z => match self.rotation % 4 {
                0 => [(0, 0), (1, 0), (1, 1), (2, 1)],
                1 => [(2, 0), (1, 1), (2, 1), (1, 2)],
                2 => [(0, 1), (1, 1), (1, 2), (2, 2)],
                _ => [(1, 0), (0, 1), (1, 1), (0, 2)],
            },
            Shape::J => match self.rotation % 4 {
                0 => [(0, 0), (0, 1), (1, 1), (2, 1)],
                1 => [(1, 0), (2, 0), (1, 1), (1, 2)],
                2 => [(0, 1), (1, 1), (2, 1), (2, 2)],
                _ => [(1, 0), (1, 1), (0, 2), (1, 2)],
            },
            Shape::L => match self.rotation % 4 {
                0 => [(2, 0), (0, 1), (1, 1), (2, 1)],
                1 => [(1, 0), (1, 1), (1, 2), (2, 2)],
                2 => [(0, 1), (1, 1), (2, 1), (0, 2)],
                _ => [(0, 0), (1, 0), (1, 1), (1, 2)],
            },
        }
    }

    /// Returns the global grid coordinates of the current piece.
    pub fn get_grid_positions(&self) -> [(i32, i32); 4] {
        let mut pos = [(0, 0); 4];
        for (i, &(dx, dy)) in self.blocks().iter().enumerate() {
            pos[i] = (self.x + dx, self.y + dy);
        }
        pos
    }

    pub fn rotate(&mut self) {
        self.rotation += 1;
    }

    pub fn revert_rotation(&mut self) {
        if self.rotation == 0 {
            self.rotation = 3;
        } else {
            self.rotation -= 1;
        }
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }
}
