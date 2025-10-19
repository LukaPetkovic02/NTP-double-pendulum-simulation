import subprocess
import time
import os
import multiprocessing as mp
import numpy as np
import matplotlib.pyplot as plt
from statistics import mean, stdev

# ======== KONFIGURACIJA ========== #
N_RUNS = 5        # broj ponavljanja
BASE_STEPS = 20000
DT = 0.001
BASE_Y0 = [np.pi/2, 0.0, np.pi/2 + 0.01, 0.0]
PARAMS = {"m1":1.0,"m2":1.0,"l1":1.0,"l2":1.0,"g":9.81}
# ================================= #

def measure_seq_time(steps):
    times = []
    for _ in range(N_RUNS):
        start = time.perf_counter()
        subprocess.run([
            "python", "simulate.py",
            "--steps", str(steps),
            "--dt", str(DT),
            "--out", "outputs/test_seq.csv"
        ], stdout=subprocess.DEVNULL)
        times.append(time.perf_counter() - start)
    return mean(times), stdev(times)

def measure_parallel_time(nproc, steps):
    times = []
    for _ in range(N_RUNS):
        cmd = [
            "python", "parallel_sim.py",
            "--nproc", str(nproc),
            "--steps", str(steps)
        ]
        env = {"OMP_NUM_THREADS": str(nproc)}
        start = time.perf_counter()
        subprocess.run(cmd, env={**env, **dict(os.environ)}, stdout=subprocess.DEVNULL)
        times.append(time.perf_counter() - start)
    return mean(times), stdev(times)

def amdahl_speedup(s, p):
    return 1 / (s + (1 - s)/p)

def gustafson_speedup(s, p):
    return s + (1 - s) * p

def run_experiments():
    os.makedirs("outputs", exist_ok=True)
    num_cores = mp.cpu_count()
    cores = [1, 2, 4, 8, 12][:num_cores]

    print("=== Početak eksperimenata ===")

    # ----- JAKO SKALIRANJE ----- #
    print("\n--- Jako skaliranje ---")
    seq_mean, seq_std = measure_seq_time(BASE_STEPS)
    print(f"Sekvencijalno vreme (steps={BASE_STEPS}): {seq_mean:.3f}s")

    strong_times = []
    for n in cores:
        mean_t, std_t = measure_parallel_time(n, BASE_STEPS)
        strong_times.append((n, mean_t, std_t))

    # Automatsko određivanje sekvencijalnog dela S
    max_proc = cores[-1]
    T_max = strong_times[-1][1]
    S = max(0.0, min(1.0, (max_proc * T_max - seq_mean) / (max_proc * T_max - seq_mean + seq_mean)))  # aproksimacija
    print(f"Aproksimirani sekvencijalni deo koda: S = {S:.3f} ({S*100:.1f}%)")

    strong_speedups = [seq_mean / t for (_, t, _) in strong_times]
    amdahl_theory = [amdahl_speedup(S, n) for n in cores]

    plt.figure(figsize=(12,6))
    plt.plot(cores, strong_speedups, "o-", label="Izmereno ubrzanje")
    plt.plot(cores, amdahl_theory, "r--", label=f"Amdahl teorija (S={S:.3f})")
    plt.xlabel("Broj jezgara")
    plt.ylabel("Ubrzanje")
    plt.title("Jako skaliranje")
    plt.grid(True)
    plt.legend()
    plt.savefig("outputs/strong_scaling.png")

    print("Tabela rezultata (jako skaliranje):")
    print("Jezgra | Prosečno vreme (s) | Std dev | Ubrzanje")
    for (n,t,std), sp in zip(strong_times,strong_speedups):
        print(f"{n:6d} | {t:18.4f} | {std:6.4f} | {sp:7.3f}")

    # ----- SLABO SKALIRANJE ----- #
    print("\n--- Slabo skaliranje ---")
    weak_times = []
    weak_speedups = []

    for n in cores:
        steps_scaled = BASE_STEPS * n
        mean_t, std_t = measure_parallel_time(n, steps_scaled)
        weak_times.append((n, mean_t, std_t))
        weak_speedups.append(gustafson_speedup(S, n))
    
    weak_speedups_measured = [seq_mean * n / t for (n, t, _) in weak_times]

    plt.figure(figsize=(12,6))
    plt.plot([n for (n,_,_) in weak_times], weak_speedups_measured, "o-", label="Izmereno (steps proporcionalno)")
    plt.plot(cores, weak_speedups, "g--", label=f"Gustafson teorija (S={S:.2f})")
    plt.xlabel("Broj jezgara")
    plt.ylabel("Ubrzanje")
    plt.title("Slabo skaliranje")
    plt.grid(True)
    plt.legend()
    plt.savefig("outputs/weak_scaling.png")

    print("Tabela rezultata (slabo skaliranje):")
    print("Jezgra | Prosečno vreme (s) | Std dev | Gustafson | Izmereno")
    for (n,t,std), sp, spm in zip(weak_times, weak_speedups, weak_speedups_measured):
        print(f"{n:6d} | {t:18.4f} | {std:6.4f} | {sp:7.3f} | {spm:10.3f}")

if __name__ == "__main__":
    run_experiments()
