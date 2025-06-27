use smallest_enclosing_circle::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// #[test]
// fn test_point_midpoint() {
//     let p1 = Point { x: 0.0, y: 0.0 };
//     let p2 = Point { x: 2.0, y: 2.0 };
//     let mid = p1.midpoint(&p2);
//     assert_eq!(mid.x, 1.0);
//     assert_eq!(mid.y, 1.0);
// }
// 
// #[test]
// fn test_point_distance() {
//     let p1 = Point { x: 0.0, y: 0.0 };
//     let p2 = Point { x: 3.0, y: 4.0 };
//     assert_eq!(p1.distance_to(&p2), 5.0);
// }
// 
// #[test]
// fn test_circle_through_3_points() {
//     let p1 = Point { x: 0.0, y: -1.0 };
//     let p2 = Point { x: 1.0, y: 0.0 };
//     let p3 = Point { x: 0.0, y: 1.0 };
//     let circle = circle_through_3_points(p1, p2, p3);
//     assert!((circle.center.x - 0.0).abs() < 1e-6);
//     assert!((circle.center.y - 0.0).abs() < 1e-6);
//     assert!((circle.radius - 1.0).abs() < 1e-6);
// }
// 
// #[test]
// fn test_is_in_circle() {
//     let center = Point { x: 0.0, y: 0.0 };
//     let c = Circle { center, radius: 1.0 };
//     let inside = Point { x: 0.5, y: 0.5 };
//     let outside = Point { x: 2.0, y: 2.0 };
//     assert!(is_in_circle(inside, &c));
//     assert!(!is_in_circle(outside, &c));
// }

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
        assert!(is_in_circle(p, &circle));
    }
}

#[test]
fn test_random_points_with_seed() {
    let mut rng = StdRng::seed_from_u64(42);
    let points: Vec<Point> = (0..100)
        .map(|_| {
            Point {
                x: rng.random_range(-100.0..100.0),
                y: rng.random_range(-100.0..100.0),
            }
        })
        .collect();

    let circle = get_min_enclosing_circle(points.clone());
    for p in points {
        assert!(is_in_circle(p, &circle));
    }
}
