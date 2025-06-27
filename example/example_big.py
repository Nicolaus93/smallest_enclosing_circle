
import numpy as np
from smallest_enclosing_circle.smallest_enclosing_circle import min_enclosing_circle

if __name__ == '__main__':
    rng = np.random.default_rng(seed=42)
    pts = rng.random((50000, 2))
    min_enclosing_circle(pts)