import numpy as np
from smallest_enclosing_circle.smallest_enclosing_circle import min_enclosing_circle
import sys
import timeit


if __name__ == '__main__':
    n_points = int(sys.argv[1])
    rng = np.random.default_rng(seed=42)
    pts = rng.random((n_points, 2))
    number = 30

    def run():
        min_enclosing_circle(pts)

    elapsed = timeit.timeit(run, number=number)
    avg = elapsed / number

    print(f"Average time over {number} runs with {n_points} points: {avg:.6f} seconds")
