#[deny(missing_docs)]

/// Encodes a position in 2D space
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// X value of the position.
    pub(crate) x: i32,
    /// Y value of the position.
    pub(crate) y: i32,
}

impl Position {
    /// Constructs a new 
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    // /// Gets the X value.
    // pub fn x(&self) -> i32 {
    //     self.x
    // }

    // /// Gets the y value.
    // pub fn y(&self) -> i32 {
    //     self.y
    // }
}