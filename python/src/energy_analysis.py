# energy_analysis.py
import csv
import sys
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path

def read_csv(path):
    t = []
    E = []
    with open(path, newline='') as f:
        r = csv.reader(f)
        header = next(r)
        for row in r:
            t.append(float(row[0]))
            E.append(float(row[5]))
    return np.array(t), np.array(E)

def analyze(path):
    t,E = read_csv(path)
    E0 = E[0]
    rel = (E - E0) / E0
    print(f"File: {path}")
    print(f"Initial E = {E0:.12f}, Final E = {E[-1]:.12f}, Rel change = {rel[-1]*100:.4f}%")
    print(f"Min/Max E during run: {E.min():.12f}, {E.max():.12f}")
    # plot
    plt.figure(figsize=(9,4))
    plt.subplot(1,2,1)
    plt.plot(t, E, label='Energy')
    plt.xlabel('t'); plt.ylabel('E'); plt.title('Energy vs time')
    plt.grid(True)
    plt.subplot(1,2,2)
    plt.plot(t, rel*100, label='Rel change (%)')
    plt.xlabel('t'); plt.ylabel('Rel E change (%)'); plt.title('Relative energy change (%)')
    plt.grid(True)
    plt.tight_layout()
    out = Path(path).with_suffix('.energy.png')
    plt.savefig(out)
    print(f"Saved plot to {out}")
    return t,E,rel

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python energy_analysis.py path/to/traj_000.csv")
        sys.exit(1)
    analyze(sys.argv[1])
