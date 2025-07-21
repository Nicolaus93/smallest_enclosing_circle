use std::time::Instant;
use rand::prelude::*;
use smallest_enclosing_circle::{get_min_enclosing_circle, welzl, Point};
use std::hint::black_box;

fn generate_test_points(n: usize, seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n)
        .map(|_| Point {
            x: rng.random_range(-100.0..100.0),
            y: rng.random_range(-100.0..100.0),
        })
        .collect()
}

fn benchmark_welzl(points: Vec<Point>, iterations: usize) -> std::time::Duration {
    // warmup
    for _ in 0..5 {
        let _ = black_box(welzl(points.clone()));
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(welzl(points.clone()));
    }
    start.elapsed()
}

fn benchmark_full_pipeline(points: Vec<Point>, iterations: usize) -> std::time::Duration {
    // warmup
    for _ in 0..5 {
        let _ = black_box(get_min_enclosing_circle(points.clone()));
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = black_box(get_min_enclosing_circle(points.clone()));
    }
    start.elapsed()
}

// Add this as a binary target
fn main() {
    println!("Rust-only benchmarking for Welzl algorithm");

    let test_sizes = vec![100000, 100000, 100000, 100000];
    let iterations = 10;

    for &size in &test_sizes {
        println!("\n=== Testing with {} points ===", size);

        let points = generate_test_points(size, 42);

        // Benchmark just the welzl algorithm
        let welzl_duration = benchmark_welzl(points.clone(), iterations);
        println!("Welzl only: {:?} per iteration", welzl_duration / iterations.try_into().unwrap());

        // Benchmark the full pipeline (including shuffle)
        let full_duration = benchmark_full_pipeline(points.clone(), iterations);
        println!("Full pipeline: {:?} per iteration", full_duration / iterations.try_into().unwrap());

        // Performance metrics
        let points_per_second = (size * iterations) as f64 / welzl_duration.as_secs_f64();
        println!("Throughput: {:.0} points/second", points_per_second);
    }

}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::hint::black_box;
    use smallest_enclosing_circle::incircle;

    #[test]
    fn bench_welzl_small() {
        let points = generate_test_points(100, 42);
        let start = Instant::now();
        for _ in 0..1000 {
            black_box(welzl(black_box(points.clone())));
        }
        println!("Small dataset: {:?}", start.elapsed() / 1000);
    }

    #[test]
    fn bench_welzl_medium() {
        let points = generate_test_points(1000, 42);
        let start = Instant::now();
        for _ in 0..100 {
            black_box(welzl(black_box(points.clone())));
        }
        println!("Medium dataset: {:?}", start.elapsed() / 100);
    }

    #[test]
    fn bench_incircle_only() {
        let points = generate_test_points(4, 42);
        let [a, b, c, d] = [points[0], points[1], points[2], points[3]];

        let start = Instant::now();
        for _ in 0..1_000_000 {
            black_box(incircle(black_box(a), black_box(b), black_box(c), black_box(d)));
        }
        println!("Incircle function: {:?}", start.elapsed() / 1_000_000);
    }
}
