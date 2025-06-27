import numpy as np
import matplotlib.pyplot as plt

from smallest_enclosing_circle import min_enclosing_circle


def get_min_enclosing_circle(pts: np.ndarray) -> tuple[float, tuple[float, float]]:
    # Call the Rust function
    center, radius = min_enclosing_circle(pts)

    # Convert to Python-native types (center is a 1D np.ndarray)
    center = tuple(center.tolist())  # (x, y)
    return radius, center


if __name__ == '__main__':
    for i in range(5):
        rng = np.random.default_rng(seed=42 + i)
        pts = rng.random((300, 2))

        circle_r, cntr = get_min_enclosing_circle(pts)

        # Circle points
        theta = np.linspace(0, 2 * np.pi, 300)
        x_circle = cntr[0] + circle_r * np.cos(theta)
        y_circle = cntr[1] + circle_r * np.sin(theta)

        plt.plot(pts[:, 0], pts[:, 1], 'o', label='Points')
        plt.plot(x_circle, y_circle, 'b-', label='Enclosing Circle')
        plt.scatter(*cntr, color='green', s=30, label='Center')
        plt.axis('equal')
        plt.legend()
        plt.title(f"Min Enclosing Circle (Seed {42 + i})")
        plt.show()
