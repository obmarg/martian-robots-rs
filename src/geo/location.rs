use super::orientation::Orientation;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<Orientation> for Point {
    type Output = Self;

    fn add(self, other: Orientation) -> Self {
        self + other.as_point()
    }
}
