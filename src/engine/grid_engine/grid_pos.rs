//! Grid coordinate primitive

use macroquad_toolkit::grid::TilePos;
use serde::{Deserialize, Serialize};

/// Grid position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Check if position is within grid bounds
    pub fn in_bounds(&self, width: u32, height: u32) -> bool {
        self.x >= 0 && self.y >= 0 && (self.x as u32) < width && (self.y as u32) < height
    }

    /// Get adjacent positions (4-directional)
    pub fn neighbors(&self) -> [GridPos; 4] {
        [
            GridPos::new(self.x - 1, self.y),
            GridPos::new(self.x + 1, self.y),
            GridPos::new(self.x, self.y - 1),
            GridPos::new(self.x, self.y + 1),
        ]
    }

    /// Manhattan distance to another position
    pub fn distance(&self, other: GridPos) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    /// Convert to array index for flat storage
    pub fn to_index(&self, width: u32) -> usize {
        (self.y as u32 * width + self.x as u32) as usize
    }

    /// Create from array index
    pub fn from_index(index: usize, width: u32) -> Self {
        Self {
            x: (index as u32 % width) as i32,
            y: (index as u32 / width) as i32,
        }
    }

    pub(crate) fn to_tile_pos(self) -> TilePos {
        TilePos::new(self.x, self.y)
    }

    pub(crate) fn from_tile_pos(pos: TilePos) -> Self {
        Self::new(pos.x, pos.y)
    }
}

#[cfg(test)]
mod tests {
    use super::GridPos;

    #[test]
    fn in_bounds_rejects_negative_and_out_of_range() {
        assert!(GridPos::new(0, 0).in_bounds(4, 4));
        assert!(GridPos::new(3, 3).in_bounds(4, 4));
        assert!(!GridPos::new(4, 0).in_bounds(4, 4));
        assert!(!GridPos::new(0, 4).in_bounds(4, 4));
        assert!(!GridPos::new(-1, 0).in_bounds(4, 4));
    }

    #[test]
    fn neighbors_are_four_directional() {
        let neighbors = GridPos::new(5, 5).neighbors();
        assert_eq!(
            neighbors,
            [
                GridPos::new(4, 5),
                GridPos::new(6, 5),
                GridPos::new(5, 4),
                GridPos::new(5, 6),
            ]
        );
    }

    #[test]
    fn distance_is_manhattan() {
        assert_eq!(GridPos::new(0, 0).distance(GridPos::new(3, 4)), 7);
        assert_eq!(GridPos::new(2, 2).distance(GridPos::new(2, 2)), 0);
    }

    #[test]
    fn index_roundtrip() {
        let width = 10;
        for i in 0..(width * 3) {
            let pos = GridPos::from_index(i as usize, width);
            assert_eq!(pos.to_index(width), i as usize);
        }
    }
}
