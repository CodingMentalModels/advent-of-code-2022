use std::ops::{Add, AddAssign, Sub, Mul};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}
impl Vec2 {

    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn i() -> Self {
        Self::new(1, 0)
    }

    pub fn j() -> Self {
        Self::new(0, 1)
    }

    pub fn get_l1_norm(&self) -> u32 {
        (self.x).abs() as u32 + (self.y).abs() as u32
    }

    pub fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    pub fn get_points_between(left: Self, right: Self) -> Vec<Self> {
        assert!(left.x() == right.x() || left.y() == right.y() || (left.x() - right.x()).abs() == (left.y() - right.y()).abs());
        let direction = (right - left).signum();
        let mut to_return = Vec::new();
        let mut next = left;
        while next != right {
            to_return.push(next);
            next += direction;
        }
        to_return.push(right);
        return to_return;
    }

}
#[cfg(test)]
mod test_vector {
    use super::*;

    #[test]
    fn test_vectors_add() {
        let x = Vec2::new(1, -2);
        let y = Vec2::new(3, 5);
        assert_eq!(x + y, Vec2::new(4, 3));
    }

    #[test]
    fn test_vectors_get_l1_norm() {
        assert_eq!(Vec2::new(0, 0).get_l1_norm(), 0);
        assert_eq!(Vec2::new(-2, 1).get_l1_norm(), 3);
        assert_eq!(Vec2::new(5, 5).get_l1_norm(), 10);
    }

    #[test]
    fn test_vectors_get_signum() {
        assert_eq!(Vec2::new(0, 0), Vec2::new(0, 0));
        assert_eq!(Vec2::new(12, 5), Vec2::new(1, 1));
        assert_eq!(Vec2::new(-3, 2), Vec2::new(-1, 1));
        assert_eq!(Vec2::new(-10, -43), Vec2::new(-1, -1));
    }

    #[test]
    fn test_vectors_get_points_between() {
        assert_eq!(Vec2::get_points_between(Vec2::i(), Vec2::i()), vec![Vec2::i()]);
        assert_eq!(Vec2::get_points_between(Vec2::j(), Vec2::j()), vec![Vec2::j()]);
        assert_eq!(
            Vec2::get_points_between(Vec2::new(1, 5), Vec2::new(3, 5)),
            vec![Vec2::new(1, 5), Vec2::new(2, 5), Vec2::new(3, 5)]
        );
        assert_eq!(
            Vec2::get_points_between(Vec2::new(1, 2), Vec2::new(1, -1)),
            vec![Vec2::new(1, 2), Vec2::new(1, 1), Vec2::new(1, 0), Vec2::new(1, -1)]
        );
        assert_eq!(
            Vec2::get_points_between(Vec2::new(1, 2), Vec2::new(3, 4)),
            vec![Vec2::new(1, 2), Vec2::new(2, 3), Vec2::new(3, 4)]
        );
        assert_eq!(
            Vec2::get_points_between(Vec2::new(1, 2), Vec2::new(-1, 4)),
            vec![Vec2::new(1, 2), Vec2::new(0, 3), Vec2::new(-1, 4)]
        );
    }
}