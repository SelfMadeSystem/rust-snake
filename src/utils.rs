use speedy2d::{dimen::IVec2, window::KeyScancode};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    pub fn is_adjacent(&self, other: Direction) -> bool {
        match self {
            Direction::Up | Direction::Down => {
                other == Direction::Left || other == Direction::Right
            }
            Direction::Left | Direction::Right => {
                other == Direction::Up || other == Direction::Down
            }
        }
    }

    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }
}

pub const KEY_UP: KeyScancode = 103;
pub const KEY_DOWN: KeyScancode = 108;
pub const KEY_LEFT: KeyScancode = 105;
pub const KEY_RIGHT: KeyScancode = 106;

pub const KEY_W: KeyScancode = 17;
pub const KEY_A: KeyScancode = 30;
pub const KEY_S: KeyScancode = 31;
pub const KEY_D: KeyScancode = 32;