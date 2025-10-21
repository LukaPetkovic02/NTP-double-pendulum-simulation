import os
os.environ["OMP_NUM_THREADS"] = "1"
os.environ["OPENBLAS_NUM_THREADS"] = "1"
os.environ["MKL_NUM_THREADS"] = "1"
os.environ["VECLIB_MAXIMUM_THREADS"] = "1"
os.environ["NUMEXPR_NUM_THREADS"] = "1"

import multiprocessing as mp
mp.set_start_method('spawn', force=True)

import time, argparse
import numpy as np
from simulate import run_single

def worker(i, y0, params, dt, steps, outdir):
    #out = os.path.join(outdir, f"traj_{i:03d}.csv")
    t = run_single(None, y0, params, dt, steps)
    return i, t

def make_initial_conditions(base, n, delta=1e-3):
    # generiše n blago različitih theta2 vrednosti
    inits = []
    for i in range(n):
        perturb = (i - n//2) * delta
        y0 = base.copy()
        y0[2] += perturb  # menja theta2
        inits.append(y0)
    return inits

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--nproc", type=int, default=mp.cpu_count(), help="Broj procesa")
    parser.add_argument("--dt", type=float, default=0.001)
    parser.add_argument("--steps", type=int, default=60000)
    args = parser.parse_args()

    outdir = "outputs/parallel"
    os.makedirs(outdir, exist_ok=True)
    dt = 0.001
    steps = 100000
    params = {"m1":1.0,"m2":1.0,"l1":1.0,"l2":1.0,"g":9.81}
    base = [np.pi/2, 0.0, np.pi/2 + 0.01, 0.0]

    N = args.nproc
    print(f"Pokrecem sa {N} procesa...")
    initials = make_initial_conditions(base, N, delta=1e-3)

    tasks = [(i, initials[i], params, dt, steps, outdir) for i in range(N)]

    start = time.perf_counter()
    with mp.Pool(processes=N) as pool:
        results = pool.starmap(worker, tasks)
    elapsed = time.perf_counter() - start

    print("Per-sim times:", results)
    print(f"Total parallel wall-clock: {elapsed:.4f}s")
