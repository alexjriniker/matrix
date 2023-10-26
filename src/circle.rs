use crate::point::Point;
use std::{f64::consts::PI, fmt::Display};

#[derive(PartialEq, Debug, Clone)]
pub struct Circle {
    pub pt: Point,
    pub radius: f64,
}

// #[derive(Debug)]
// pub enum CircleIntersection {
//     Intersection((Point, Point)),
//     Outside,
//     Inside,
// }

#[derive(Debug)]
pub enum CircleIntersectionError {
    /// The intersection object is inside the current circle
    InsideErr,
    /// The intersection object completely contains the current circle, with no overlap
    ContainsErr,
    /// The intersection object has no overlap with the current circle
    NoOverlapErr,
}
type CircleCircleIntersection = (Point, Point);

use CircleIntersectionError::*;

#[allow(dead_code)]
impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            pt: Point::new(x, y),
            radius,
        }
    }

    pub fn scale(&mut self, factor: f64) -> () {
        self.radius *= factor;
    }

    pub fn area(&self) -> f64 {
        self.radius.powi(2) * PI
    }

    pub fn circumference(&self) -> f64 {
        2.0 * self.radius * PI
    }

    pub fn contains_pt(&self, pt: Point) -> bool {
        self.pt.distance_to(&pt) <= self.radius
    }

    pub fn contains_circle(&self, circle: Circle) -> bool {
        let dist = self.pt.distance_to(&circle.pt);

        self.radius + dist < circle.radius
    }

    // TODO: fix circle intersection method!!!!!!!!!
    pub fn intersection(
        &self,
        circle: &Circle,
    ) -> Result<CircleCircleIntersection, CircleIntersectionError> {
        let mut delta = &circle.pt - &self.pt;
        let dist = delta.pow(2).sum().sqrt();

        if dist > self.radius + circle.radius {
            Err(NoOverlapErr)
        } else if dist < (self.radius - circle.radius).abs() {
            Err(ContainsErr)
        } else {
            delta = &delta / dist;

            let a = (self.radius.powi(2) - circle.radius.powi(2) + dist.powi(2)) / (2.0 * dist);
            let pt = &self.pt + &(&delta * a);
            let h = (self.radius.powi(2) - a.powi(2)).sqrt();

            // println!(
            //     "X: {}",
            //     self.pt.x + a * (circle.pt.x - self.pt.x) / dist
            //         - h * (circle.pt.y - self.pt.y) / dist
            // );
            // println!("INT X: {}", self.pt.x + a * delta.x - h * delta.y);
            // println!(
            //     "X1: {}",
            //     self.pt.x
            //         + a * (circle.pt.x - self.pt.x) / dist
            //         + h * (circle.pt.y - self.pt.y) / dist
            // );

            // let inverter = Point::new(-1.0, 1.0);
            delta = &delta * h; //&(&delta * h)* &inverter).invert() ;
            delta = delta.invert();
            delta = &delta * &Point::new(1.0, -1.0);

            // let v = vec![1, 23];
            // let sa = v.iter().reduce(|a, x| x);

            let pt_a = &pt + &delta;
            // println!("PT A: {:?}", pt_a);

            Ok((pt_a, &pt - &delta))
        }
        // let dist = circle.pt.distance_to(&circle.pt);

        // /*if self.radius + dist < circle.radius {
        //     println!("inside error");
        //     Err(InsideErr)
        // // } else if circle.radius + dist < self.radius {
        // //     println!("Contains error");

        // //     Err(ContainsErr)
        // } else if dist < (self.radius - circle.radius).abs() {
        //     println!("Nooverlap error");

        //     Err(NoOverlapErr)
        // } else */
        // {
        //     let mut delta = &(&circle.pt - &self.pt) / dist;

        //     let a = (self.radius.powi(2) - circle.radius.powi(2) + dist.powi(2)) / (2.0 * dist);
        //     let h = (self.radius.powi(2) - a.powi(2)).sqrt();

        //     let pt = &self.pt + &(&delta * a);

        //     delta = &(&delta * h).invert() * &Point::new(1.0, -1.0);

        //     let intersection_pt_1 = &pt + &delta;
        //     let intersection_pt_2 = &pt - &delta;

        //     Ok((intersection_pt_1, intersection_pt_2))
        // }
    }

    // pub fn intersection(&self, circle: &Circle) -> CircleIntersection {
    //     let mut delta = &circle.pt - &self.pt;
    //     let dist = delta.pow(2).sum().sqrt();

    //     if dist > self.radius + circle.radius {
    //         Outside
    //     } else if dist < (self.radius - circle.radius).abs() {
    //         Inside
    //     } else {
    //         delta = &delta / dist;

    //         let a = (self.radius.powi(2) - circle.radius.powi(2) + dist.powi(2)) / (2.0 * dist);
    //         let pt = &self.pt + &(&delta * a);
    //         let h = (self.radius.powi(2) - a.powi(2)).sqrt();

    //         // println!(
    //         //     "X: {}",
    //         //     self.pt.x + a * (circle.pt.x - self.pt.x) / dist
    //         //         - h * (circle.pt.y - self.pt.y) / dist
    //         // );
    //         // println!("INT X: {}", self.pt.x + a * delta.x - h * delta.y);
    //         // println!(
    //         //     "X1: {}",
    //         //     self.pt.x
    //         //         + a * (circle.pt.x - self.pt.x) / dist
    //         //         + h * (circle.pt.y - self.pt.y) / dist
    //         // );

    //         // let inverter = Point::new(-1.0, 1.0);
    //         delta = &delta * h; //&(&delta * h)* &inverter).invert() ;
    //         delta = delta.invert();
    //         delta = &delta * &Point::new(1.0, -1.0);

    //         // let v = vec![1, 23];
    //         // let sa = v.iter().reduce(|a, x| x);

    //         let pt_a = &pt + &delta;
    //         // println!("PT A: {:?}", pt_a);

    //         Intersection((pt_a, &pt - &delta))
    //     }
    // }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.pt, self.radius)
    }
}
