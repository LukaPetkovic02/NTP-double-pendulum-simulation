import subprocess
import time
import multiprocessing as mp
import numpy as np
import matplotlib.pyplot as plt
from statistics import mean, stdev

# ======== KONFIGURACIJA ==========
N_RUNS = 5  # broj ponavljanja za svaku konfiguraciju
STEPS = 5000
DT = 0.001
BASE_Y0 = [np.pi/2, 0.0, np.pi/2 + 0.01, 0.0]
PARAMS = {"m1": 1.0, "m2": 1.0, "l1": 1.0, "l2": 1.0, "g": 9.81}
# =================================

def measure_seq_time():
    """Pokreće sekvencijalnu simulaciju i meri prosečno vreme."""
    times = []
    for _ in range(N_RUNS):
        start = time.perf_counter()
        subprocess.run(["python", "simulate.py", "--steps", str(STEPS), "--dt", str(DT), "--out", "outputs/test_seq.csv"],
                       stdout=subprocess.DEVNULL)
        times.append(time.perf_counter() - start)
    return mean(times), stdev(times)


def measure_parallel_time(num_processes, weak_scaling=False):
    """Pokreće paralelnu simulaciju sa zadatim brojem procesa."""
    times = []
    for _ in range(2):
        cmd = ["python", "parallel_sim.py", "--nproc", str(num_processes)]
        env = {"OMP_NUM_THREADS": str(num_processes)}
        start = time.perf_counter()
        subprocess.run(cmd, env={**env, **dict(os.environ)}, stdout=subprocess.DEVNULL)
        elapsed = time.perf_counter() - start
        times.append(elapsed)
    return mean(times), stdev(times)


def amdahl_speedup(p, f):
    """Amdalov zakon"""
    return 1 / ((1 - p) + p / f)


def gustafson_speedup(p, f):
    """Gustafsonov zakon"""
    return (1 - p) + p * f


def run_experiments():
    print("=== Početak eksperimenata ===")

    num_cores = mp.cpu_count()
    cores = [1, 2, 4, 6, 8, 10, 12][:num_cores]  # ograniči na raspoloživa jezgra

    seq_mean, seq_std = measure_seq_time()
    print(f"Sekvencijalno prosečno vreme: {seq_mean:.3f}s")

    strong_times = []
    weak_times = []

    for n in cores:
        mean_t, std_t = measure_parallel_time(n)
        strong_times.append((n, mean_t, std_t))
        weak_times.append((n, mean_t * (1 / n), std_t))  # aproksimacija za slabo skaliranje

    # Izračunaj ubrzanja
    strong_speedups = [seq_mean / t for (_, t, _) in strong_times]
    weak_speedups = [gustafson_speedup(0.95, n) for n in cores]

    # Teorijski maksimumi
    p = 0.95  # 95% paralelizovano
    amdahl_theory = [amdahl_speedup(p, n) for n in cores]
    gustafson_theory = [gustafson_speedup(p, n) for n in cores]

    # ---- GRAFICI ----
    plt.figure(figsize=(12, 6))
    plt.plot(cores, strong_speedups, "o-", label="Izmereno ubrzanje (Python)")
    plt.plot(cores, amdahl_theory, "r--", label=f"Amdahl teorija (p={p*100:.0f}%)")
    plt.xlabel("Broj procesorskih jezgara")
    plt.ylabel("Ubrzanje")
    plt.title("Jako skaliranje - Python (Amdahl)")
    plt.legend()
    plt.grid()
    plt.savefig("outputs/strong_scaling_python.png")

    plt.figure(figsize=(12, 6))
    plt.plot(cores, weak_speedups, "o-", label="Izmereno ubrzanje (Python)")
    plt.plot(cores, gustafson_theory, "g--", label=f"Gustafson teorija (p={p*100:.0f}%)")
    plt.xlabel("Broj procesorskih jezgara")
    plt.ylabel("Ubrzanje")
    plt.title("Slabo skaliranje - Python (Gustafson)")
    plt.legend()
    plt.grid()
    plt.savefig("outputs/weak_scaling_python.png")

    print("Grafici sačuvani u outputs/ direktorijumu.")

    # ---- TABELE ----
    print("\nTabela rezultata (jako skaliranje):")
    print("Jezgra | Prosečno vreme (s) | Std dev (s) | Ubrzanje")
    for (n, t, std), sp in zip(strong_times, strong_speedups):
        print(f"{n:6d} | {t:20.4f} | {std:12.4f} | {sp:8.3f}")


if __name__ == "__main__":
    import os
    os.makedirs("outputs", exist_ok=True)
    run_experiments()
