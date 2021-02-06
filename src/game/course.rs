#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x as f64,
            y: y as f64,
        }
    }
}

#[derive(Clone)]
pub struct Course {
    pub points: std::vec::Vec<Point>,
}

impl Course {
    pub fn default() -> Course {
        Course {
            points: vec![
                Point::new(0.0, 0.0),
                Point::new(300.0, 0.0),
                Point::new(300.0, 300.0),
                Point::new(-300.0, 300.0),
                Point::new(-300.0, 0.0),
            ],
        }
    }
}
