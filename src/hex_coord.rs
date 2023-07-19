use std::ops::Mul;

use super::Coord;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HexCoord(pub f32, pub f32);


const Y_FACTOR: f32 = 0.866025403784;

impl From<Coord> for HexCoord {
    fn from(value: Coord) -> Self {
        HexCoord(
            value.0 as f32 + 0.5 * value.1 as f32,
            value.1 as f32 * Y_FACTOR
        )
    }
}

impl Mul<f32> for HexCoord {
    type Output = HexCoord;

    fn mul(self, rhs: f32) -> Self::Output {
        HexCoord(self.0 * rhs, self.1 * rhs)
    }
}