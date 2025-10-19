import argparse, csv, os, time
import numpy as np
from dynamics import derivatives, energy
from rk4 import integrate

def run_single(output_path, y0, params, dt, steps):
    start = time.perf_counter()
    times, traj, energies = integrate(y0, dt, steps, derivatives, params, record_energy_fn=energy)
    elapsed = time.perf_counter() - start

    if output_path:
        os.makedirs(os.path.dirname(output_path), exist_ok=True)
        with open(output_path, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["t","theta1","omega1","theta2","omega2","energy"])
            for i in range(len(times)):
                t = times[i]
                th1, om1, th2, om2 = traj[i]
                en = energies[i] if energies is not None else ""
                writer.writerow([t, th1, om1, th2, om2, en])

    return elapsed

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", default="outputs/traj_000.csv")
    parser.add_argument("--dt", type=float, default=0.001)
    parser.add_argument("--steps", type=int, default=60000)
    args = parser.parse_args()

    # početni uslovi: theta1, omega1, theta2, omega2
    y0 = [np.pi/2, 0.0, np.pi/2 + 0.01, 0.0]
    params = {"m1":1.0,"m2":1.0,"l1":1.0,"l2":1.0,"g":9.81}

    t = run_single(args.out, y0, params, args.dt, args.steps)
    print(f"Finished. time={t:.4f}s -> wrote {args.out}")
