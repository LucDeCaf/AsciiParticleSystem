#[derive(Debug, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vector2<f32> {
    pub fn floor(&self) -> Self {
        let mut floored = self.clone();
        floored.x = self.x.floor();
        floored.y = self.y.floor();
        floored
    }

    pub fn ceil(&self) -> Self {
        let mut floored = self.clone();
        floored.x = self.x.ceil();
        floored.y = self.y.ceil();
        floored
    }

    pub fn round(&self) -> Self {
        let mut floored = self.clone();
        floored.x = self.x.round();
        floored.y = self.y.round();
        floored
    }
}
