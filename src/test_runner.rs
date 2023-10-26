use std::fmt::Display;

use rand::{distributions::Uniform, rngs::ThreadRng, Rng};

use crate::{point::Point, rectangle::Rectangle};

#[derive(Debug)]
pub struct TestResult {
    pub tag_pt: Point,
    pub anchor_pts: Vec<Point>,
    pub real_distances: Vec<f64>,
    pub distance_coefficients: Vec<f64>,
    pub adjusted_distances: Vec<f64>,
    pub predicted_pt: Point,
    pub delta: f64,
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tag Pt: {}", self.tag_pt)?;

        let anchors_str = self
            .anchor_pts
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(f, "Anchor Pts: {}", anchors_str)?;

        let distances_str = self
            .real_distances
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(f, "Real Distances: {}", distances_str)?;

        let coefficients_str = self
            .distance_coefficients
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(f, "Distances Coefficients: {}", coefficients_str)?;

        let adjusted_str = self
            .adjusted_distances
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(f, "Adjusted Distances: {}", adjusted_str)?;

        writeln!(f, "Predicted Pt: {}", self.predicted_pt)?;
        writeln!(f, "Delta: {}", self.delta)
    }
}

pub type TestRunnerCallback = fn(&Vec<Point>, &Vec<f64>) -> Point;

#[derive(Debug)]
pub struct TestRunner {
    pub num_of_anchors: i32,
    pub error_margin: f64,
    pub callback: TestRunnerCallback,
    x_range: Uniform<f64>,
    y_range: Uniform<f64>,
    rnd: ThreadRng,
}

impl TestRunner {
    pub fn new(
        num_of_anchors: i32,
        error_margin: f64,
        bounds: Rectangle,
        callback: TestRunnerCallback,
    ) -> Self {
        TestRunner {
            num_of_anchors,
            error_margin,
            callback,
            x_range: Uniform::from(bounds.x_range()),
            y_range: Uniform::from(bounds.y_range()),
            rnd: rand::thread_rng(),
        }
    }

    fn rand_pt(&mut self) -> Point {
        Point::new(
            self.rnd.sample(&self.x_range),
            self.rnd.sample(&self.y_range),
        )
    }

    pub fn run(&mut self, times: i32) -> Vec<TestResult> {
        let mut tag_pt = self.rand_pt();
        let mut anchor_pts = (0..self.num_of_anchors)
            .into_iter()
            .map(|_| self.rand_pt())
            .collect::<Vec<_>>();

        (1..=times)
            .into_iter()
            .map(|time| {
                print!(
                    "\rTest #{} / {}, ({:.2}%)",
                    time,
                    times,
                    time as f64 / times as f64 * 100.0
                );

                let distances = anchor_pts
                    .iter()
                    .map(|pt| pt.distance_to(&tag_pt))
                    .collect::<Vec<_>>();
                let distance_coefficients = (0..self.num_of_anchors)
                    .into_iter()
                    .map(|_| self.rnd.gen_range(-self.error_margin..self.error_margin))
                    .collect::<Vec<_>>();
                let adjusted_distances = distances
                    .iter()
                    .enumerate()
                    .map(|(index, dist)| dist + dist * distance_coefficients[index])
                    .collect::<Vec<_>>();

                let result = (self.callback)(&anchor_pts, &adjusted_distances);

                let delta = tag_pt.distance_to(&result);

                let result = TestResult {
                    tag_pt: tag_pt.clone(),
                    anchor_pts: anchor_pts.clone(),
                    real_distances: distances,
                    distance_coefficients,
                    adjusted_distances,
                    predicted_pt: result,
                    delta,
                };

                // println!("{}", result);

                if time != times {
                    tag_pt = self.rand_pt();

                    for pt in anchor_pts.iter_mut() {
                        *pt = self.rand_pt();
                    }
                } else {
                    println!()
                }

                result
            })
            .collect::<Vec<_>>()
    }
}
