use std::{fmt::Display, ops::RangeInclusive};

use crate::{
    circle::Circle,
    point::Point,
    two_dim_shape::{Contains, TwoDimShape},
};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub pt: Point,
    pub width: f64,
    pub height: f64,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rectangle {
            pt: Point::new(x, y),
            width,
            height,
        }
    }

    pub fn x_range(&self) -> RangeInclusive<f64> {
        self.pt.x..=(self.pt.x + self.width)
    }

    pub fn y_range(&self) -> RangeInclusive<f64> {
        self.pt.y..=(self.pt.y + self.height)
    }
}

impl TwoDimShape for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn calc_center_pt(&self) -> Point {
        &self.pt + &(&Point::new(self.width, self.height) / 2.0)
    }

    fn calc_min_span(&self) -> f64 {
        self.width.min(self.height)
    }

    fn calc_max_span(&self) -> f64 {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

impl Contains for Rectangle {
    fn contains(&self, obj: &Self) -> bool {
        self.pt.x < obj.pt.x
            && self.pt.y < obj.pt.y
            && self.pt.x + self.width > obj.pt.x + obj.width
            && self.pt.y + self.height > obj.pt.y + obj.height
    }
}

impl Contains<Point> for Rectangle {
    fn contains(&self, obj: &Point) -> bool {
        self.pt.x < obj.x
            && self.pt.y < obj.y
            && self.pt.x + self.width > obj.x
            && self.pt.y + self.height > obj.y
    }
}

impl Contains<Circle> for Rectangle {
    fn contains(&self, obj: &Circle) -> bool {
        let delta_x = Point::new(obj.radius, 0.0);
        let delta_y = Point::new(0.0, obj.radius);

        [
            &(&obj.pt + &delta_x),
            &(&obj.pt - &delta_x),
            &(&obj.pt + &delta_x),
            &{ &obj.pt - &delta_y },
        ]
        .iter()
        .all(|&pt| self.contains(pt))
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}+{}", self.pt, self.width, self.height)
    }
}
