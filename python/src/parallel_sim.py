# parallel_sim.py
import os
os.environ["OMP_NUM_THREADS"]="1"; os.environ["OPENBLAS_NUM_THREADS"]="1"
os.environ["MKL_NUM_THREADS"]="1"; os.environ["VECLIB_MAXIMUM_THREADS"]="1"
os.environ["NUMEXPR_NUM_THREADS"]="1"

import multiprocessing as mp
mp.set_start_method('spawn', force=True)

import time, argparse, numpy as np
from simulate import run_single

def worker(i, y0, params, dt, steps):
    t = run_single(None, y0, params, dt, steps)
    return i, t

def make_initial_conditions(base, n, delta=1e-3):
    inits = []
    for i in range(n):
        y = base.copy()
        y[2] += (i - n//2) * delta
        inits.append(y)
    return inits

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--nproc", type=int, default=mp.cpu_count())
    parser.add_argument("--dt", type=float, default=0.001)
    parser.add_argument("--steps", type=int, default=60000)
    parser.add_argument("--jobs", type=int, default=32, help="Ukupan broj trajektorija (K)")
    parser.add_argument("--mode", choices=["strong","weak"], default="strong")
    args = parser.parse_args()

    params = {"m1":1.0,"m2":1.0,"l1":1.0,"l2":1.0,"g":9.81}
    base   = [np.pi/2, 0.0, np.pi/2 + 0.01, 0.0]

    K_total = args.jobs if args.mode=="strong" else args.jobs * args.nproc

    initials = make_initial_conditions(base, K_total, delta=1e-3)
    tasks = [(i, initials[i], params, args.dt, args.steps) for i in range(K_total)]

    start = time.perf_counter()
    with mp.Pool(processes=args.nproc) as pool:
        results = pool.starmap(worker, tasks)
    elapsed = time.perf_counter() - start

    per_times = [t for _, t in results]
    print(f"Mode={args.mode} | K_total={K_total} | nproc={args.nproc}")
    print(f"Per-sim mean={np.mean(per_times):.4f}s std={np.std(per_times):.4f}s")
    print(f"Total wall-clock: {elapsed:.4f}s")
