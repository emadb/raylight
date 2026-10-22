pub struct Map {
    tiles: [[u8; crate::params::MAP_WIDTH];crate::params::MAP_HEIGHT]
}

impl Map {
    pub fn build(tiles: [[u8; crate::params::MAP_WIDTH];crate::params::MAP_HEIGHT]) -> Self {
        Self { tiles }
    }

    pub fn get(&self, x: i32, y: i32) -> u8 {
        let r = y as usize;
        let c = x as usize;
        self.tiles[r][c]
    }
}