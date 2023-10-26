use std::fs;

// use circle::{Circle, CircleIntersectionError};
use matrix::Matrix;
use rectangle::Rectangle;
use test_runner::TestRunner;

mod circle;
mod matrix;
mod point;
mod rectangle;
mod test_runner;
mod two_dim_shape;

use crate::{point::Point, two_dim_shape::TwoDimShape};

fn paper_way(anchors: &Vec<Point>, distances: &Vec<f64>) -> Point {
    let n = anchors.len();

    let mut a = Matrix::new(n - 1, 2);
    let mut b = Matrix::new(n - 1, 1); // vec![0.0; n];

    for i in 1..n {
        *a.get_mut(i - 1, 0).unwrap() = anchors[i].x - anchors[0].x;
        *a.get_mut(i - 1, 1).unwrap() = anchors[i].y - anchors[0].y;

        *b.get_mut(i - 1, 0).unwrap() = distances[0].powi(2) - distances[i].powi(2)
            + anchors[i].pow(2).sum()
            - anchors[0].pow(2).sum();
    }

    let a_t = a.transpose();
    let result = a_t
        .dot(&a)
        .unwrap()
        .invert()
        .unwrap()
        .dot(&a_t)
        .unwrap()
        .dot(&(&b / 2.0))
        .unwrap();

    assert!(result.n_rows == 2);
    assert!(result.n_cols == 1);

    Point::new(*result.get(0, 0).unwrap(), *result.get(1, 0).unwrap())
}

/*
fn circle_way(anchors: &Vec<Point>, distances: &Vec<f64>) -> Point {
    let distances = distances.iter().map(|v| v * 1.02).collect::<Vec<_>>(); // Scale all distances by `1.2`

    let n = anchors.len();
    let mut intersections: Vec<Vec<Point>> = vec![];
    let circles = anchors
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let /*mut*/ circle = Circle::new(x.x, x.y, distances[i]);
            // circle.scale(1.0 + inaccuracy * 3.0);

            circle
        })
        .collect::<Vec<_>>();

    for i in 0..n {
        for j in (i + 1)..n {
            let c1 = &circles[i];
            let c2 = &circles[j];

            let intersection = match c1.intersection(c2) {
                Err(err) => match err {
                    _ => {
                        // println!("Escaping...");
                        return Point::new(0.0, 0.0);
                    } // get me out of here!!!!!!
                },
                Ok(a) => a,
            };
            // println!("Intersection: {:?}", intersection);
            // println!("{:?}", intersection);
            intersections.push(vec![intersection.0, intersection.1]);
        }
    }

    let mut scores = vec![0.0, 0.0];

    //     println!("===================");
    //     println!("TIME: {}", time);
    //     println!("===================");

    for i in 1..n {
        // for 1, 2
        // for j in 0..2 {
        let a_1 = intersections[0][0].distance_to(&intersections[i][0]);
        let a_2 = intersections[0][0].distance_to(&intersections[i][1]);

        let a = a_1.min(a_2);

        let b_1 = intersections[0][1].distance_to(&intersections[i][0]);
        let b_2 = intersections[0][1].distance_to(&intersections[i][1]);

        let b = b_1.min(b_2);

        // println!("a_1, a_2, b_1, b_2: {}, {}, {}, {}", a_1, a_2, b_1, b_2);

        let idx = if a < b { 0 } else { 1 };
        scores[idx] += 1.0;

        // for j in 0..2 {
        //     for k in 0..2 {
        //         scores[j] += intersections[0][j].distance_to(&intersections[i][k])
        //             - intersections[0][j].distance_to(&intersections[i][k]);
        //     }
        // }
    }

    let index = scores
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap();

    intersections[0][index].clone()

    // Point::new(0.0, 0.0)
    //     println!("Index: {}", index);

    //     let d1 = tag.distance_to(&intersections[0][0]);
    //     let d2 = tag.distance_to(&intersections[0][1]);

    //     let correct = if d1 < d2 { 0 } else { 1 };

    //     // println!("Correct: {}", correct);
    //     println!("Tag: {}", tag);
    //     // println!("PTS: {:?}", &intersections)

    //     println!();
    //     println!("ANCHORS: {}, {}, {}", a1, a2, a3);
    //     println!(
    //         "ANCHOR DISTS: {}, {}, {}",
    //         a1.distance_to(&a2),
    //         a1.distance_to(&a3),
    //         a2.distance_to(&a3)
    //     );
    //     println!();
    //     println!("DISTANCES: {:?}", distances);
    //     println!();
    //     for i in 0..3 {
    //         let pt = &vec![&a1, &a2, &a3][i];
    //         let dist = distances[i];

    //         println!("FORMULA: {}^2=(x-{})^2+(y-{})^2", dist, pt.x, pt.y);
    //     }
    //     println!();

    //     println!("INTERACTIONS:");
    //     for v in intersections {
    //         println!("{:?}", v);
    //     }

    //     println!("SCORES: {:?}", scores);
    //     // if index != correct {

    //     // }

    //     assert_eq!(index, correct, "wrong index");
} */

fn calculate_mean(numbers: &Vec<f64>) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn calculate_median(numbers: &Vec<f64>) -> f64 {
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted_numbers.len() / 2;
    if sorted_numbers.len() % 2 == 0 {
        (sorted_numbers[mid - 1] + sorted_numbers[mid]) / 2.0
    } else {
        sorted_numbers[mid]
    }
}

fn calculate_std(numbers: &Vec<f64>) -> f64 {
    let mean = calculate_mean(numbers);
    let variance =
        numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (numbers.len() as f64 - 1.0);
    variance.sqrt()
}

fn identify_outliers(numbers: &Vec<f64>, threshold: f64) -> Vec<f64> {
    let mean = calculate_mean(numbers);
    let std = calculate_std(numbers);
    let z_scores: Vec<f64> = numbers.iter().map(|&x| (x - mean) / std).collect();

    z_scores
        .iter()
        .zip(numbers.iter())
        .filter(|(z_score, _)| z_score.abs() > threshold)
        .map(|(_, &x)| x)
        .collect()
}

fn main() {
    let num_of_anchors = 3;
    let error_margin = 0.05;
    let bounds = Rectangle::new(100.0, 100.0, 500.0, 500.0);
    let callback = paper_way;

    let mut runner = TestRunner::new(num_of_anchors, error_margin, bounds.clone(), callback);

    let times = 10_000_000;
    let result: Vec<test_runner::TestResult> = runner.run(times);

    let deltas = result.iter().map(|v| v.delta);
    let vec = deltas.clone().collect::<Vec<_>>();

    let mean = calculate_mean(&vec);
    let median = calculate_median(&vec);
    let std = calculate_std(&vec);
    let outliers = identify_outliers(&vec, 2.0).len();

    println!("\nMean: {:.2}", mean);
    println!("Median: {:.2}", median);
    println!("Standard Deviation: {:.2}", std);

    println!(
        "\nNum Outliers: {} / {} ({:.2}%)",
        outliers,
        times,
        (outliers as f64 / times as f64) * 100.0
    );

    let completely_wrong = vec
        .iter()
        .filter(|v| **v > bounds.calc_max_span())
        .collect::<Vec<_>>()
        .len();

    println!(
        "\nCompletely Wrong: {} / {} ({:.2}%)",
        completely_wrong,
        times,
        completely_wrong as f64 / times as f64 * 100.0
    );
    let n_non_compliant_5 = vec
        .iter()
        .filter(|v| {
            v.partial_cmp(&&(bounds.calc_max_span() * 0.05))
                .unwrap()
                .is_gt()
        })
        .collect::<Vec<_>>()
        .len();
    let n_non_compliant_10 = vec
        .iter()
        .filter(|v| {
            v.partial_cmp(&&(bounds.calc_max_span() * 0.10))
                .unwrap()
                .is_gt()
        })
        .collect::<Vec<_>>()
        .len();
    let n_non_compliant_20 = vec
        .iter()
        .filter(|v| {
            v.partial_cmp(&&(bounds.calc_max_span() * 0.20))
                .unwrap()
                .is_gt()
        })
        .collect::<Vec<_>>()
        .len();
    println!();
    println!(
        "Non Compliant (5%): {} / {} ({:.2}%)",
        n_non_compliant_5,
        times,
        (n_non_compliant_5 as f64 / times as f64) * 100.0
    );
    println!(
        "Non Compliant (10%): {} / {} ({:.2}%)",
        n_non_compliant_10,
        times,
        (n_non_compliant_10 as f64 / times as f64) * 100.0
    );
    println!(
        "Non Compliant (20%): {} / {} ({:.2}%)",
        n_non_compliant_20,
        times,
        (n_non_compliant_20 as f64 / times as f64) * 100.0
    );

    fs::write(
        "./out.csv",
        deltas
            // .filter(|v| ((v - mean) / std).abs() < 2.0)
            .map(|v| (100.0 * &(v / bounds.calc_max_span())).to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
    .expect("Bad file write");

    // let inaccuracy = 0.05;
    // let (lower_range, upper_range) = (100.0, 300.0);

    // let mut rnd = rand::thread_rng();

    // let a1 = Point::new(
    //     rnd.gen_range(lower_range..=upper_range),
    //     rnd.gen_range(lower_range..=upper_range),
    // );
    // let a2 = Point::new(
    //     rnd.gen_range(lower_range..=upper_range),
    //     rnd.gen_range(lower_range..=upper_range),
    // );
    // let a3 = Point::new(
    //     rnd.gen_range(lower_range..=upper_range),
    //     rnd.gen_range(lower_range..=upper_range),
    // );

    // let tag = Point::new(
    //     rnd.gen_range(lower_range..=upper_range),
    //     rnd.gen_range(lower_range..=upper_range),
    // );

    // let anchors = &vec![&a1, &a2, &a3];
    // let distances = anchors
    //     .iter()
    //     .map(|p| {
    //         let dist = p.distance_to(&tag);

    //         rnd.gen_range((dist - dist * inaccuracy)..=(dist + dist * inaccuracy))
    //     })
    //     .collect::<Vec<_>>();

    // let mut a: Vec<Vec<f64>> = vec![vec![0.0; 2]; 2];
    // let mut b: Vec<f64> = vec![0.0; 2];

    // for i in 1..3 {
    //     a[i - 1][0] = anchors[i].x - anchors[0].x;
    //     a[i - 1][1] = anchors[i].y - anchors[0].y;
    //     b[i - 1] = distances[0].powi(2) - distances[i].powi(2) + anchors[i].pow(2).sum()
    //         - anchors[0].pow(2).sum();
    // }

    // let mut det = a[0][0] * a[1][1] - a[1][0] * a[0][1];

    // det = 1.0 / det;

    // let mut a_inv: Vec<Vec<f64>> = vec![vec![0.0; 2]; 2];

    // a_inv[0][0] = det * a[1][1];
    // a_inv[0][1] = -det * a[0][1];
    // a_inv[1][0] = -det * a[1][0];
    // a_inv[1][1] = det * a[0][0];

    // let x = 0.5 * (a_inv[0][0] * b[0] + a_inv[0][1] * b[1]);
    // let y = 0.5 * (a_inv[1][0] * b[0] + a_inv[1][1] * b[1]);

    // println!("TAG: {}", tag);
    // println!("RESULT: ({}, {})", x, y);
    // println!("DELTA: {}", (&Point::new(x, y) - &tag).pow(2).sum().sqrt());

    // println!("Matrix A: {:?}", a);
    // println!("DET: {}", det);

    // let inaccuracy = 0.05;
    // let (lower_range, upper_range) = (100.0, 300.0);

    // let mut rnd = rand::thread_rng();

    // 'abc: for time in 0..100 {
    //     let a1 = Point::new(
    //         rnd.gen_range(lower_range..=upper_range),
    //         rnd.gen_range(lower_range..=upper_range),
    //     );
    //     let a2 = Point::new(
    //         rnd.gen_range(lower_range..=upper_range),
    //         rnd.gen_range(lower_range..=upper_range),
    //     );
    //     let a3 = Point::new(
    //         rnd.gen_range(lower_range..=upper_range),
    //         rnd.gen_range(lower_range..=upper_range),
    //     );

    //     if a1.distance_to(&a2) < 60.0 || a1.distance_to(&a3) < 60.0 || a2.distance_to(&a3) < 60.0 {
    //         println!("Continuing because of anchor distance!");
    //         continue 'abc;
    //     }

    //     let tag = Point::new(
    //         rnd.gen_range(lower_range..=upper_range),
    //         rnd.gen_range(lower_range..=upper_range),
    //     );

    //     let distances = vec![&a1, &a2, &a3]
    //         .iter()
    //         .map(|p| {
    //             let dist = p.distance_to(&tag);

    //             rnd.gen_range((dist - dist * inaccuracy)..=(dist + dist * inaccuracy))
    //         })
    //         .collect::<Vec<_>>();

    //     let circles = vec![&a1, &a2, &a3]
    //         .iter()
    //         .enumerate()
    //         .map(|(i, x)| {
    //             let /*mut*/ circle = Circle::new(x.x, x.y, distances[i]);
    //             // circle.scale(1.0 + inaccuracy * 3.0);

    //             circle
    //         })
    //         .collect::<Vec<_>>();

    //     let mut intersections: Vec<Vec<Point>> = vec![];

    //     for i in 0..2 {
    //         for j in (i + 1)..3 {
    //             let c1 = &circles[i];
    //             let c2 = &circles[j];

    //             let intersection = match c1.intersection(c2) {
    //                 CircleIntersection::Inside => continue 'abc, //panic!("Circle inside other circle"),
    //                 CircleIntersection::Outside => continue 'abc, //panic!("Circle outside other circle!"),
    //                 CircleIntersection::Intersection(ar) => ar,
    //             };

    //             // println!("{:?}", intersection);
    //             intersections.push(vec![intersection.0, intersection.1]);
    //         }
    //     }

    //     let mut scores = vec![0.0, 0.0];

    //     println!("===================");
    //     println!("TIME: {}", time);
    //     println!("===================");

    //     for i in 1..3 {
    //         // for 1, 2
    //         // for j in 0..2 {
    //         let a_1 = intersections[0][0].distance_to(&intersections[i][0]);
    //         let a_2 = intersections[0][0].distance_to(&intersections[i][1]);

    //         let a = a_1.min(a_2);

    //         let b_1 = intersections[0][1].distance_to(&intersections[i][0]);
    //         let b_2 = intersections[0][1].distance_to(&intersections[i][1]);

    //         let b = b_1.min(b_2);

    //         println!("a_1, a_2, b_1, b_2: {}, {}, {}, {}", a_1, a_2, b_1, b_2);

    //         let idx = if a < b { 0 } else { 1 };
    //         scores[idx] += 1.0; // if d < g { g } else { d };
    //                             // scores[j] = 1.0;
    //                             // scores[j] = 2.2;

    //         for j in 0..2 {
    //             for k in 0..2 {
    //                 scores[j] += intersections[0][j].distance_to(&intersections[i][k])
    //                     - intersections[0][j].distance_to(&intersections[i][k]);
    //             }
    //         }
    //         // }
    //     }

    //     let index = scores
    //         .iter()
    //         .enumerate()
    //         .max_by(|(_, a), (_, b)| a.total_cmp(b))
    //         .map(|(index, _)| index)
    //         .unwrap();
    //     println!("Index: {}", index);

    //     let d1 = tag.distance_to(&intersections[0][0]);
    //     let d2 = tag.distance_to(&intersections[0][1]);

    //     let correct = if d1 < d2 { 0 } else { 1 };

    //     // println!("Correct: {}", correct);
    //     println!("Tag: {}", tag);
    //     // println!("PTS: {:?}", &intersections)

    //     println!();
    //     println!("ANCHORS: {}, {}, {}", a1, a2, a3);
    //     println!(
    //         "ANCHOR DISTS: {}, {}, {}",
    //         a1.distance_to(&a2),
    //         a1.distance_to(&a3),
    //         a2.distance_to(&a3)
    //     );
    //     println!();
    //     println!("DISTANCES: {:?}", distances);
    //     println!();
    //     for i in 0..3 {
    //         let pt = &vec![&a1, &a2, &a3][i];
    //         let dist = distances[i];

    //         println!("FORMULA: {}^2=(x-{})^2+(y-{})^2", dist, pt.x, pt.y);
    //     }
    //     println!();

    //     println!("INTERACTIONS:");
    //     for v in intersections {
    //         println!("{:?}", v);
    //     }

    //     println!("SCORES: {:?}", scores);
    //     // if index != correct {

    //     // }

    //     assert_eq!(index, correct, "wrong index");
    // } *****************************************888

    // 0 1
    // 0 2
    // 1 3
    // println!("Distances: {:?}", distances);

    // let c1 = Circle::new(a1.x, a1.y, distances[0]);
    // let c2 = Circle::new(a2.x, a2.y, distances[1]);

    // println!("{:?}", c1.intersection(&c2));

    // let mat_1 = Matrix::from_iter(
    //     3,
    //     6,
    //     [
    //         1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //         1.0,
    //     ],
    // );

    // let deltas = vec![&a1, &a2, &a3]
    //     .into_iter()
    //     .map(|pt| pt - &tag)
    //     .collect::<Vec<Point>>();

    // let dt_1 = &deltas[0];
    // let dt_2 = &deltas[1];
    // let dt_3 = &deltas[2];

    // let mat_2 = Matrix::from_iter(
    //     6,
    //     1,
    //     vec![dt_1.x, dt_1.y, dt_2.x, dt_2.y, dt_3.x, dt_3.y]
    //         .into_iter()
    //         .map(|value| value.powi(2)),
    // );

    // // println!("{:?}", deltas);

    // let result = match mat_1.dot(&mat_2) {
    //     Some(v) => v,
    //     None => panic!("Bad matrix dimensions"),
    // };

    // println!(
    //     "Distances: {:?}",
    //     distances
    //         .iter()
    //         .map(|value| value.powi(1))
    //         .collect::<Vec<_>>()
    // );
    // println!("Result: {:?}", result);

    // println!("\n\n\n");
    // println!("{}, {}", a1.x - dt_1.pow(2).x.sqrt(), a1.x);

    // test_pt();

    // let a = Point::new(10.0, 10.0);
    // let b = Point::new(2.0, 6.0);

    // println!("Point A: {:?}", a);
    // println!("Point B: {:?}", b);

    // let c = a + &b;

    // println!("Point C: {}", c);

    // // println!("Hello, world!");

    // let a = Matrix::from_iter(3, 3, [6.0, 2.0, 4.0, -1.0, 4.0, 3.0, -2.0, 9.0, 3.0]);

    // // println!("{:?}", a);
    // let b = Matrix::from_iter(3, 1, [4.0, -2.0, 1.0]);

    // let c = match &a + &b {
    //     Some(v) => v,
    //     None => panic!("Bad Matrix Dimensions For Adding"),
    // };
    // let d = match &a + &b {
    //     Some(v) => v,
    //     None => panic!("Bad Matrix Dimensions For Adding"),
    // };

    // println!("{:?}", c);
    // println!("{:?}", d);

    // let c = match a.dot(&b) {
    //     None => {
    //         println!("Shit hit the fan");
    //         panic!();
    //     }
    //     Some(v) => v,
    // };

    // println!("{:?}", c);
}
