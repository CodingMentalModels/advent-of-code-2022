use super::vector::Vec2;



#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {

    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "left" | "l" => Ok(Self::Left),
            "right" | "r" => Ok(Self::Right),
            "down" | "d" => Ok(Self::Down),
            "up" | "u" => Ok(Self::Up),
            _ => Err(format!("Unable to parse Direction from string: {}", s))
        }
    }

    pub fn get_delta(&self) -> Vec2 {
        match self {
            Self::Left => Vec2::new(-1, 0),
            Self::Right => Vec2::new(1, 0),
            Self::Down => Vec2::new(0, -1),
            Self::Up => Vec2::new(0, 1),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

}