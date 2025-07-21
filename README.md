# Smallest Enclosing Circle (Welzl's Algorithm in Rust)

A fast and robust implementation of Welzl’s algorithm for computing the **smallest enclosing circle** of a set of 2D points, written in Rust with optional Python bindings via PyO3.

---

## 🚀 Features

- Compute the minimal circle that encloses a given set of 2D points.
- Uses a randomized version of **Welzl's algorithm**.
- Includes robust geometric predicates (`incircle`, `orient2d`) for numerical stability.
- Simple and efficient `Point` and `Circle` data types.
- Optional **Python bindings** using `PyO3` and `numpy`.

---

## 📦 Installation

### Rust (Library/Crate)

Add to your `Cargo.toml`:

```toml
[dependencies]
smallest_enclosing_circle = { path = "path/to/this/repo" }
```

### Python (via PyO3)

Build the Python module using maturin:

```
maturin develop  # or maturin build
```
Then you can import it in Python: 

```python
from smallest_enclosing_circle import min_enclosing_circle
```

## Usage

### Rust

```rust
use smallest_enclosing_circle::{Point, get_min_enclosing_circle};

fn main() {
    let points = vec![
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.0, y: 1.0 },
        Point { x: -1.0, y: 0.0 },
        Point { x: 0.0, y: -1.0 },
    ];

    let circle = get_min_enclosing_circle(points);
    println!("Center: ({}, {}), Radius: {}", circle.center.x, circle.center.y, circle.radius);
}
```

### Python

```python
import numpy as np
from smallest_enclosing_circle import min_enclosing_circle

points = np.array([
[1.0, 0.0],
[0.0, 1.0],
[-1.0, 0.0],
[0.0, -1.0],
])

center, radius = min_enclosing_circle(points)
print("Center:", center)
print("Radius:", radius)
```

📚 References

- [Welzl's Algorithm](https://en.wikipedia.org/wiki/Smallest-circle_problem)
- [Shewchuk Robust Geometric Predicates](https://people.eecs.berkeley.edu/~jrs/papers/robust-predicates.pdf)


🙌 Acknowledgments

This project was inspired by https://www.nayuki.io/page/smallest-enclosing-circle, which is great but unfortunately still
missing a Rust implementation. The primary source for the implementation of the iterative version of Welzl's algorithm came 
from [this great answer on Stack Overflow](https://stackoverflow.com/a/69430104).
