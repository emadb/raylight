pub struct Map {
    tiles: [[u8; crate::params::MAP_WIDTH];crate::params::MAP_HEIGHT]
}

impl Map {
    pub fn build(tiles: [[u8; crate::params::MAP_WIDTH];crate::params::MAP_HEIGHT]) -> Self {
        Self { tiles }
    }
}