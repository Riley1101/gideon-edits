use super::Position;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<Locaton> for Position {
    fn from(loc: Locaton) -> Self {
        Self { x: loc.x, y: loc.y }
    }
}
