use std::fmt::Display;

use crate::{circle::Circle, point::Point, rectangle::Rectangle};

pub trait Contains<T = Self> {
    fn contains(&self, obj: &T) -> bool;
}

pub trait TwoDimShape: Contains<Point> + Contains<Rectangle> + Contains<Circle> + Display {
    /// Returns the shapes area
    fn calc_area(&self) -> f64;
    /// Returns the shapes perimeter
    fn calc_perimeter(&self) -> f64;

    /// Returns shapes center point
    fn get_center_pt(&self) -> Point;

    /// Returns the length of the smallest line that can be drawn inside the object
    fn calc_min_span(&self) -> f64;
    /// Returns the length of the biggest line that can be drawn inside this object
    fn calc_max_span(&self) -> f64;
}
