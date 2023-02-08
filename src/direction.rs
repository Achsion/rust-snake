use crossterm::event::KeyCode;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

impl Direction {
    pub fn from_key_code(key_code: KeyCode) -> Option<Direction> {
        return match key_code {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        };
    }

    pub fn check_opposite(dir1: Direction, dir2: Direction) -> bool {
        return dir1 == Direction::Up && dir2 == Direction::Down
            || dir1 == Direction::Down && dir2 == Direction::Up
            || dir1 == Direction::Left && dir2 == Direction::Right
            || dir1 == Direction::Right && dir2 == Direction::Left;
    }
}
