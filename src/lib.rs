use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use rand::rng;
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray2};
use rand::prelude::*; // needed for shuffle, rng
use numpy::PyUntypedArrayMethods;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    #[inline]
    pub fn distance_squared_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    #[inline]
    pub fn distance_to(&self, other: &Point) -> f64 {
        self.distance_squared_to(other).sqrt()
    }
}


const EPSILON: f64 = 1e-12;

// Shewchuk's incircle from https://people.eecs.berkeley.edu/~jrs/papers/robust-predicates.pdf
#[inline]
pub fn incircle(a: Point, b: Point, c: Point, d: Point) -> bool {
    // translate points to the origin
    let adx = a.x - d.x;
    let ady = a.y - d.y;
    let bdx = b.x - d.x;
    let bdy = b.y - d.y;
    let cdx = c.x - d.x;
    let cdy = c.y - d.y;

    let adist = adx * adx + ady * ady;
    let bdist = bdx * bdx + bdy * bdy;
    let cdist = cdx * cdx + cdy * cdy;

    let det = adx * (bdy * cdist - cdy * bdist)
        - ady * (bdx * cdist - cdx * bdist)
        + adist * (bdx * cdy - bdy * cdx);

    det >= -EPSILON
}


// To determine the orientation of three points a, b, c, we only need to compute the 2D 
// determinant of the two segments formed by those points. This is the classic orient2d predicate.
#[inline]
fn orient2d(pa: Point, pb: Point, pc: Point) -> f64 {
    (pa.x - pc.x) * (pb.y - pc.y) - (pa.y - pc.y) * (pb.x - pc.x)
}


fn point_in_circle(p: Point, boundary: &[Point]) -> bool {
    match boundary.len() {
        0 | 1 => false,
        2 => {
            // For 2 points, use circle defined by diameter
            let a = boundary[0];
            let b = boundary[1];
            let center_x = (a.x + b.x) * 0.5;
            let center_y = (a.y + b.y) * 0.5;

            let dx = p.x - center_x;
            let dy = p.y - center_y;
            let dist_sq = dx * dx + dy * dy;

            let radius_sq = a.distance_squared_to(&b) * 0.25;
            dist_sq <= radius_sq + EPSILON
        }
        3 => {
            let a = boundary[0];
            let b = boundary[1];
            let c = boundary[2];

            let orientation = orient2d(a, b, c);
            if orientation.abs() < EPSILON {
                // Degenerate triangle: colinear → no valid circle
                false
            } else if orientation > 0.0 {
                incircle(a, b, c, p)
            } else {
                incircle(c, b, a, p)
            }
        }
        _ => unreachable!("Boundary should not exceed 3 points"),
    }
}


pub fn welzl(points: Vec<Point>) -> Vec<Point> {
    let n = points.len();
    if n == 0 {
        return Vec::new();
    }

    // Use indices instead of collecting points repeatedly
    let mut circle_idxs: Vec<usize> = Vec::with_capacity(3);
    let mut i = 0;

    while i < n {
        // Check if current point is already in boundary or inside current circle
        let already_in_boundary = circle_idxs.contains(&i);

        if !already_in_boundary {
            // Build boundary points slice without allocation
            let mut boundary_points = [Point { x: 0.0, y: 0.0 }; 3];
            let boundary_len = circle_idxs.len();

            for (idx, &boundary_idx) in circle_idxs.iter().enumerate() {
                boundary_points[idx] = points[boundary_idx];
            }

            let point_inside = point_in_circle(
                points[i],
                &boundary_points[..boundary_len]
            );

            if !point_inside {
                // Remove points that come before current index
                circle_idxs.retain(|&j| j > i);
                circle_idxs.push(i);
                i = if circle_idxs.len() < 3 { 0 } else { i + 1 };
                continue;
            }
        }

        i += 1;
    }
    
    circle_idxs.iter().map(|&j| points[j]).collect()
}


pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

fn circle_through_3_points(p1: Point, p2: Point, p3: Point) -> Circle {
    let ax = p1.x;
    let ay = p1.y;
    let bx = p2.x;
    let by = p2.y;
    let cx = p3.x;
    let cy = p3.y;

    let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
    if d.abs() < 1e-12 {
        panic!("Points are colinear or too close for reliable circumcircle");
    }

    let ux = ((ax.powi(2) + ay.powi(2)) * (by - cy)
        + (bx.powi(2) + by.powi(2)) * (cy - ay)
        + (cx.powi(2) + cy.powi(2)) * (ay - by)) / d;

    let uy = ((ax.powi(2) + ay.powi(2)) * (cx - bx)
        + (bx.powi(2) + by.powi(2)) * (ax - cx)
        + (cx.powi(2) + cy.powi(2)) * (bx - ax)) / d;

    let center = Point { x: ux, y: uy };
    let radius = center.distance_to(&p1);

    Circle { center, radius }
}


fn circle_from(points: &[Point]) -> Circle {
    match points.len() {
        0 => Circle { center: Point { x: 0.0, y: 0.0 }, radius: 0.0 },
        1 => Circle { center: points[0], radius: 0.0 },
        2 => {
            let center = points[0].midpoint(&points[1]);
            let radius = points[0].distance_to(&center);
            Circle { center, radius }
        }
        3 => circle_through_3_points(points[0], points[1], points[2]),
        _ => unreachable!(),
    }
}


// pub fn is_in_circle(p: Point, c: &Circle) -> bool {
//     p.distance_to(&c.center) <= c.radius + 1e-12
// }
// 
// 
// fn welzl_recursive(points: &mut Vec<Point>, boundary: &mut Vec<Point>, depth: usize) -> Circle {
//     assert!(depth < 10_000, "Recursion too deep");  // or log it
// 
//     if points.is_empty() || boundary.len() == 3 {
//         return circle_from(boundary);
//     }
// 
//     let p = points.pop().unwrap();
//     let circle = welzl_recursive(points, boundary, depth + 1);
// 
//     if is_in_circle(p, &circle) {
//         points.push(p);
//         circle
//     } else {
//         boundary.push(p);
//         let result = welzl_recursive(points, boundary, depth + 1);
//         boundary.pop();
//         points.push(p);
//         result
//     }
// }
 
pub fn get_min_enclosing_circle(mut points: Vec<Point>) -> Circle {
    points.shuffle(&mut rng());
    // welzl_recursive(&mut points, &mut vec![], 0)
    let circle_points = welzl(points);
    circle_from(&circle_points)
}

#[pyfunction]
fn min_enclosing_circle<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray2<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, f64)> {
    let shape = arr.shape();
    if shape.len() != 2 || shape[1] != 2 {
        return Err(PyValueError::new_err("Expected Nx2 array of (x, y) coordinates"));
    }

    let points: Vec<Point> = arr
        .as_array()
        .rows()
        .into_iter()
        .map(|row| Point { x: row[0], y: row[1] })
        .collect();

    let circle = get_min_enclosing_circle(points);
    let center = vec![circle.center.x, circle.center.y].into_pyarray(py);
    Ok((center.to_owned(), circle.radius))
}

#[pymodule]
fn smallest_enclosing_circle<'py>(_py: Python<'py>, m: Bound<'py, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(min_enclosing_circle, m.clone())?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    #[test]
    fn test_point_midpoint() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 2.0, y: 2.0 };
        let mid = p1.midpoint(&p2);
        assert_eq!(mid.x, 1.0);
        assert_eq!(mid.y, 1.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_circle_through_3_points() {
        let p1 = Point { x: 0.0, y: -1.0 };
        let p2 = Point { x: 1.0, y: 0.0 };
        let p3 = Point { x: 0.0, y: 1.0 };
        let circle = circle_through_3_points(p1, p2, p3);
        assert!((circle.center.x - 0.0).abs() < 1e-6);
        assert!((circle.center.y - 0.0).abs() < 1e-6);
        assert!((circle.radius - 1.0).abs() < 1e-6);
    }

    fn is_in_circle(p: Point, c: &Circle) -> bool {
        p.distance_to(&c.center) <= c.radius + 1e-12
    }

    #[test]
    fn test_is_in_circle() {
        let center = Point { x: 0.0, y: 0.0 };
        let c = Circle { center, radius: 1.0 };
        let inside = Point { x: 0.5, y: 0.5 };
        let outside = Point { x: 2.0, y: 2.0 };
        assert!(is_in_circle(inside, &c));
        assert!(!is_in_circle(outside, &c));
    }

    #[test]
    fn test_welzl_deterministic() {
        let points = vec![
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.0, y: 1.0 },
            Point { x: -1.0, y: 0.0 },
            Point { x: 0.0, y: -1.0 },
        ];
        let circle = get_min_enclosing_circle(points.clone());
        for p in points {
            assert!(p.distance_to(&circle.center) <= circle.radius + 1e-12);
        }
    }

    #[test]
    fn test_random_points_with_seed() {
        let mut rng = StdRng::seed_from_u64(42);
        let points: Vec<Point> = (0..100)
            .map(|_| Point {
                x: rng.random_range(-100.0..100.0),
                y: rng.random_range(-100.0..100.0),
            })
            .collect();

        let circle = get_min_enclosing_circle(points.clone());
        for p in points {
            assert!(p.distance_to(&circle.center) <= circle.radius + 1e-12);
        }
    }
}
