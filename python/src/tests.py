import subprocess
import numpy as np
import matplotlib.pyplot as plt
import time
import statistics
import csv
import os

# --- KONFIGURACIJA EKSPERIMENATA ---
REPEATS = 30
PROCESSES = [1, 2, 4, 8, 16]
DT = 0.001
STEPS_BASE = 60000
JOBS = 16
F_SEQ = 0.05
OUTDIR = "results"

os.makedirs(OUTDIR, exist_ok=True)


# --- POMOĆNE FUNKCIJE ---
def run_command(cmd):
    start = time.perf_counter()
    subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return time.perf_counter() - start


def amdahl_speedup(p, f_seq):
    return 1.0 / (f_seq + (1.0 - f_seq) / p)


def gustafson_speedup(p, f_seq):
    return p - f_seq * (p - 1.0)


def strong_scaling():
    print("\n--- JAKO SKALIRANJE (Python) ---")
    results = []
    for nproc in PROCESSES:
        times = []
        for _ in range(REPEATS):
            if nproc == 1:
                cmd = ["python", "parallel_sim.py",
                       "--nproc", "1", "--mode", "strong",
                       "--jobs", str(JOBS), "--dt", str(DT), "--steps", str(STEPS_BASE)]
            else:
                cmd = ["python", "parallel_sim.py",
                       "--nproc", str(nproc), "--mode", "strong",
                       "--jobs", str(JOBS), "--dt", str(DT), "--steps", str(STEPS_BASE)]
            t = run_command(cmd)
            times.append(t)
        mean_t = statistics.mean(times)
        std_t = statistics.stdev(times) if len(times) > 1 else 0.0
        results.append((nproc, mean_t, std_t))
        print(f"{nproc} procesa → mean={mean_t:.4f}s std={std_t:.4f}s")
    return results


def weak_scaling():
    print("\n--- SLABO SKALIRANJE (Python) ---")
    results = []
    for nproc in PROCESSES:
        times = []
        for _ in range(REPEATS):
            cmd = ["python", "parallel_sim.py",
                   "--nproc", str(nproc), "--mode", "weak",
                   "--jobs", str(JOBS), "--dt", str(DT), "--steps", str(STEPS_BASE)]
            t = run_command(cmd)
            times.append(t)
        mean_t = statistics.mean(times)
        std_t = statistics.stdev(times) if len(times) > 1 else 0.0
        results.append((nproc, mean_t, std_t))
        print(f"{nproc} procesa → mean={mean_t:.4f}s std={std_t:.4f}s")
    return results


def save_results(filename, results, type_):
    path = os.path.join(OUTDIR, filename)
    with open(path, "w", newline="") as f:
        writer = csv.writer(f)
        if type_ == "strong":
            writer.writerow(["nproc", "mean_time", "std_dev", "speedup", "amdahl", "ideal"])
        else:
            writer.writerow(["nproc", "mean_time", "std_dev", "scaled_speedup", "gustafson", "ideal"])

        base_time = results[0][1]
        for nproc, mean_t, std_t in results:
            if type_ == "strong":
                speedup = base_time / mean_t
                amdahl = amdahl_speedup(nproc, F_SEQ)
                ideal = nproc
                writer.writerow([nproc, mean_t, std_t, speedup, amdahl, ideal])
            else:
                scaled_speedup = nproc * base_time / mean_t
                gustafson = gustafson_speedup(nproc, F_SEQ)
                ideal = nproc
                writer.writerow([nproc, mean_t, std_t, scaled_speedup, gustafson, ideal])
    print(f"{type_.capitalize()} results saved to {path}")


def plot_scaling(results, f_seq, type_):
    nprocs = [r[0] for r in results]
    times = [r[1] for r in results]
    base_time = times[0]

    plt.figure(figsize=(7, 5))

    if type_ == "strong":
        exp_speedup = [base_time / t for t in times]
        amdahl = [amdahl_speedup(p, f_seq) for p in nprocs]
        plt.plot(nprocs, exp_speedup, "o-", label="Eksperimentalni podaci")
        plt.plot(nprocs, amdahl, "r--", label=f"Amdalov zakon (f={f_seq})")
        plt.title("Jako skaliranje (Python, ensemble)")
    else:
        exp_speedup = [p * base_time / t for p, t in zip(nprocs, times)]
        gustafson = [gustafson_speedup(p, f_seq) for p in nprocs]
        plt.plot(nprocs, exp_speedup, "o-", label="Eksperimentalni podaci")
        plt.plot(nprocs, gustafson, "r--", label=f"Gustafsonov zakon (f={f_seq})")
        plt.title("Slabo skaliranje (Python, ensemble)")

    plt.plot(nprocs, nprocs, "k:", label="Idealno skaliranje")
    plt.xlabel("Broj procesa")
    plt.ylabel("Ubrzanje (Speedup)")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.savefig(os.path.join(OUTDIR, f"{type_}_scaling.png"))
    plt.close()
    print(f"Saved plot {type_}_scaling.png")


if __name__ == "__main__":
    strong = strong_scaling()
    weak = weak_scaling()

    save_results("strong_scaling.csv", strong, "strong")
    save_results("weak_scaling.csv", weak, "weak")

    plot_scaling(strong, F_SEQ, "strong")
    plot_scaling(weak, F_SEQ, "weak")

    print("\nEksperimenti završeni. Rezultati su u folderu 'results'.")
