fn main() {}

struct Rectangle {
    width: f32,
    heigit: f32,
}

struct Circle {
    radius: f32,
}

trait CalcArea {
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f32 {
        self.width * self.heigit
    }
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
}
