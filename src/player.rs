struct Direction(u16);

impl Direction {
    pub fn new(degrees: u16) -> Self {
        Direction(degrees % 360)
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

pub struct Player {
    x: u32,
    y: u32,
    direction: Direction
}

impl Player {
    pub fn new(x: u32, y: u32, direction: u16) -> Self {
        Self { x, y, direction: Direction(direction) }
    }
}