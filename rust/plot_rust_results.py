import pandas as pd
import matplotlib.pyplot as plt
import os

# 👇 Putanje do CSV fajlova koje generišu tvoji Rust eksperimenti
STRONG_FILE = "results_rust_strong.csv"
WEAK_FILE = "results_rust_weak.csv"

# 📁 Folder za slike
OUTDIR = "results_rust_plots"
os.makedirs(OUTDIR, exist_ok=True)


def plot_scaling(df, title, outname, law_label):
    """
    Crta eksperimentalni, teorijski i idealni speedup.
    """
    nprocs = df["nproc"]
    exp = df["speedup"]
    theory = df[df.columns[4]]  # Amdahl ili Gustafson
    ideal = df["ideal"]

    plt.figure(figsize=(7, 5))
    plt.plot(nprocs, exp, "o-", label="Eksperimentalni podaci", linewidth=2)
    plt.plot(nprocs, theory, "r--", label=f"{law_label}")
    plt.plot(nprocs, ideal, "k:", label="Idealno skaliranje")

    plt.title(title)
    plt.xlabel("Broj niti")
    plt.ylabel("Ubrzanje (Speedup)")
    plt.grid(True)
    plt.legend()
    plt.tight_layout()

    outpath = os.path.join(OUTDIR, outname)
    plt.savefig(outpath, dpi=300)
    plt.close()
    print(f"✅ Sačuvan grafikon: {outpath}")


def main():
    # --- Jako skaliranje ---
    if os.path.exists(STRONG_FILE):
        df_strong = pd.read_csv(STRONG_FILE)
        plot_scaling(
            df_strong,
            "Jako skaliranje (Rust)",
            "strong_scaling_rust.png",
            "Amdalov zakon"
        )
    else:
        print(f"⚠️ Nije pronađen fajl: {STRONG_FILE}")

    # --- Slabo skaliranje ---
    if os.path.exists(WEAK_FILE):
        df_weak = pd.read_csv(WEAK_FILE)
        plot_scaling(
            df_weak,
            "Slabo skaliranje (Rust)",
            "weak_scaling_rust.png",
            "Gustafsonov zakon"
        )
    else:
        print(f"⚠️ Nije pronađen fajl: {WEAK_FILE}")


if __name__ == "__main__":
    main()
