use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinates {
    x: u32,
    y: u32,
    z: u32,
}

impl Coordinates {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn z(&self) -> u32 {
        self.z
    }

    // It converts a linear index to Barycentric coordinates (x, y, z)
    pub fn from_index(index: u32, board_size: u32) -> Self {
        // As i = (r * (r + 1)) / 2
        // r = floor((sqrt(8*i + 1) - 1) / 2)
        let i_f = index as f64;
        let r = (((8.0 * i_f + 1.0).sqrt() - 1.0) / 2.0).floor() as u32;

        let row_start_index = (r * (r + 1)) / 2;
        let c = index - row_start_index;

        let x = board_size - 1 - r;
        let y = c;
        let z = (board_size - 1) - x - y;

        Coordinates::new(x, y, z)
    }

    pub fn to_index(&self, board_size: u32) -> u32 {
        let r = (board_size - 1) - self.x;
        let row_start_index = (r * (r + 1)) / 2;
        let c = self.y;
        row_start_index + c
    }

    pub fn from_vec(coords: &[u32]) -> Option<Self> {
        if coords.len() != 3 {
            return None;
        }
        Some(Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }

    pub fn touches_side_a(&self) -> bool {
        self.x == 0
    }

    pub fn touches_side_b(&self) -> bool {
        self.y == 0
    }

    pub fn touches_side_c(&self) -> bool {
        self.z == 0
    }
}

impl From<Coordinates> for Vec<u32> {
    fn from(coords: Coordinates) -> Self {
        vec![coords.x, coords.y, coords.z]
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates_conversion() {
        let coords = Coordinates::new(1, 2, 3);
        let index = coords.to_index(7);
        let converted = Coordinates::from_index(index, 7);
        assert_eq!(coords, converted);
    }

    #[test]
    fn test_coordinates_sides() {
        let coords_a = Coordinates::new(0, 2, 2);
        let coords_b = Coordinates::new(2, 0, 2);
        let coords_c = Coordinates::new(2, 2, 0);
        assert!(coords_a.touches_side_a());
        assert!(coords_b.touches_side_b());
        assert!(coords_c.touches_side_c());
    }
}
