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

    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub struct Circle {
    center: Point,
    radius: f64,
}

fn circle_through_3_points(p1: Point, p2: Point, p3: Point) -> Circle {
    let (a, b) = (p1.x, p1.y);
    let (c, d) = (p2.x, p2.y);
    let (x0, y0) = (p3.x, p3.y);

    let numerator = (x0 - a) * (x0 - c) + (y0 - b) * (y0 - d);
    let denominator = (b - d) * x0 - (a - c) * y0 + a * d - b * c;
    let alpha = numerator / denominator;

    let xc = ((a + c) + alpha * (b - d)) / 2.0;
    let yc = ((b + d) - alpha * (a - c)) / 2.0;
    let r2 = alpha * (a * d - b * c) - a * c - b * d + xc.powi(2) + yc.powi(2);
    let r = r2.sqrt();

    Circle {
        center: Point { x: xc, y: yc },
        radius: r,
    }
}

pub fn is_in_circle(p: Point, c: &Circle) -> bool {
    p.distance_to(&c.center) <= c.radius + 1e-12
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

fn welzl(points: &mut Vec<Point>, boundary: &mut Vec<Point>, depth: usize) -> Circle {
    assert!(depth < 10_000, "Recursion too deep");  // or log it

    if points.is_empty() || boundary.len() == 3 {
        return circle_from(boundary);
    }

    let p = points.pop().unwrap();
    let circle = welzl(points, boundary, depth + 1);

    if is_in_circle(p, &circle) {
        points.push(p);
        circle
    } else {
        boundary.push(p);
        let result = welzl(points, boundary, depth + 1);
        boundary.pop();
        points.push(p);
        result
    }
}

pub fn get_min_enclosing_circle(mut points: Vec<Point>) -> Circle {
    points.shuffle(&mut rng());
    welzl(&mut points, &mut vec![], 0)
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
