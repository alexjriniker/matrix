use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance_to(&self, pt: &Point) -> f64 {
        let delta: Point = self - pt;

        delta.pow(2).sum().sqrt()
    }

    pub fn pow(&self, amount: i32) -> Point {
        Point::new(self.x.powi(amount), self.y.powi(amount))
    }

    pub fn sum(&self) -> f64 {
        self.x + self.y
    }

    pub fn invert(&self) -> Point {
        Point::new(self.y, self.x)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f64> for &Point {
    type Output = Point;

    fn add(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f64> for &Point {
    type Output = Point;

    fn sub(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Div<&Point> for &Point {
    type Output = Point;

    fn div(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f64> for &Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<&Point> for &Point {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/*fn test_pt() -> () {
    // Test Variables
    let (x1, y1) = (3.2, 2.0);
    let (x2, y2) = (1.0, 3.0);

    // Point Variables
    let (pt_1, pt_2);
    let mut pt_3;

    // Construct New Points
    pt_1 = Point::new(x1, y1);
    pt_2 = Point::new(x2, y2);

    // Test Point Display Method
    assert_eq!(
        format!("{}", pt_1),
        format!("({x1}, {y1})"),
        "Point Format Failed Test"
    );

    // Test Point PartialEq
    assert!(pt_1 != pt_2, "Point Partial Eq Different Point Failed Test");
    assert!(pt_1 == pt_1, "Point Partial Eq Self Failed Test");
    assert!(
        pt_1 == Point::new(3.2, 2.0),
        "Point Partial Eq Identical Values Failed Test"
    );

    // Test Point Add
    pt_3 = &pt_1 + &pt_2;
    assert!(
        pt_3 == Point::new(x1 + x2, y1 + y2),
        "Point Add Failed Test"
    );

    // Test Point Sub
    pt_3 = &pt_1 - &pt_2;
    assert!(
        pt_3 == Point::new(x1 - x2, y1 - y2),
        "Point Sub Failed Test"
    );

    // Test Point Div
    pt_3 = &pt_1 / &pt_2;
    assert!(
        pt_3 == Point::new(x1 / x2, y1 / y2),
        "Point Div Failed Test"
    );

    // Test Point Mul
    pt_3 = &pt_1 * &pt_2;
    assert!(
        pt_3 == Point::new(x1 * x2, y1 * y2),
        "Point Mul Failed Test"
    );
}

fn test_mat() -> () {

}
 */
