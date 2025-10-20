import subprocess
import numpy as np
import matplotlib.pyplot as plt
import time
import statistics
import csv
import os

REPEATS = 30
PROCESSES = [1, 2, 4, 8]  # broj procesa za eksperiment
DT = 0.001
STEPS_BASE = 60000  # osnovna veličina posla za 1 proces

# Procena sekvencijalnog dela (f_s)
F_SEQ = 0.05  # 5% koda se ne može paralelizovati

OUTDIR = "results"
os.makedirs(OUTDIR, exist_ok=True)


def run_command(cmd):
    """Pokreće komandu i vraća vreme izvršavanja (u sekundama)."""
    start = time.perf_counter()
    subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return time.perf_counter() - start


def strong_scaling():
    """Eksperiment jakog skaliranja – isti posao, više procesa."""
    results = []
    print("\n--- STRONG SCALING ---")
    for nproc in PROCESSES:
        times = []
        for _ in range(REPEATS):
            if nproc == 1:
                cmd = ["python", "simulate.py", "--dt", str(DT), "--steps", str(STEPS_BASE)]
            else:
                cmd = ["python", "parallel_sim.py", "--nproc", str(nproc), "--dt", str(DT), "--steps", str(STEPS_BASE)]
            t = run_command(cmd)
            times.append(t)
        mean_t = statistics.mean(times)
        std_t = statistics.stdev(times) if len(times) > 1 else 0.0
        results.append((nproc, mean_t, std_t))
        print(f"{nproc} proc -> mean={mean_t:.4f}s std={std_t:.4f}s")
    return results


def weak_scaling():
    """Eksperiment slabog skaliranja – konstantan posao po procesu."""
    results = []
    print("\n--- WEAK SCALING ---")
    for nproc in PROCESSES:
        times = []
        steps = STEPS_BASE * nproc
        for _ in range(REPEATS):
            if nproc == 1:
                cmd = ["python", "simulate.py", "--dt", str(DT), "--steps", str(steps)]
            else:
                cmd = ["python", "parallel_sim.py", "--nproc", str(nproc), "--dt", str(DT), "--steps", str(steps)]
            t = run_command(cmd)
            times.append(t)
        mean_t = statistics.mean(times)
        std_t = statistics.stdev(times) if len(times) > 1 else 0.0
        results.append((nproc, mean_t, std_t))
        print(f"{nproc} proc -> mean={mean_t:.4f}s std={std_t:.4f}s")
    return results


def amdahl_speedup(p, f_seq):
    return 1.0 / (f_seq + (1 - f_seq) / p)


def gustafson_speedup(p, f_seq):
    return p - f_seq * (p - 1)


def save_results(filename, results, type_):
    path = os.path.join(OUTDIR, filename)
    with open(path, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["nproc", "mean_time", "std_dev", "speedup"])
        base_time = results[0][1]
        for nproc, mean_t, std_t in results:
            speedup = base_time / mean_t
            writer.writerow([nproc, mean_t, std_t, speedup])
    print(f"{type_} results saved to {path}")


def plot_scaling(results, f_seq, type_):
    nprocs = [r[0] for r in results]
    times = [r[1] for r in results]
    base_time = times[0]
    exp_speedup = [base_time / t for t in times]

    amdahl = [amdahl_speedup(p, f_seq) for p in nprocs]
    gustafson = [gustafson_speedup(p, f_seq) for p in nprocs]
    ideal = nprocs

    plt.figure(figsize=(7, 5))
    if type_ == "strong":
        plt.plot(nprocs, exp_speedup, "o-", label="Eksperimentalni podaci")
        plt.plot(nprocs, amdahl, "r--", label=f"Amdalov zakon (f={f_seq})")
        plt.plot(nprocs, ideal, "k:", label="Idealno skaliranje")
        plt.title("Jako skaliranje (Python)")
    else:
        plt.plot(nprocs, exp_speedup, "o-", label="Eksperimentalni podaci")
        plt.plot(nprocs, gustafson, "r--", label=f"Gustafsonov zakon (f={f_seq})")
        plt.plot(nprocs, ideal, "k:", label="Idealno skaliranje")
        plt.title("Slabo skaliranje (Python)")

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

    save_results("strong_scaling.csv", strong, "Strong")
    save_results("weak_scaling.csv", weak, "Weak")

    plot_scaling(strong, F_SEQ, "strong")
    plot_scaling(weak, F_SEQ, "weak")

    print("\nEksperimenti završeni. Rezultati su u folderu 'results'.")
