use std::process::Command;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

fn main() {
    let processes = [1, 2, 4, 8, 16];
    let repeats = 10;
    let f_seq = 0.04_f64;

    let mut file = File::create("results_rust_strong.csv").expect("cannot create file");
    writeln!(file, "nproc,mean_time,std_dev,speedup,amdahl,ideal").unwrap();

    println!("--- STRONG SCALING (Rust Optimized) ---");
    let mut seq_time = 0.0;

    for &nproc in &processes {
        let mut times = Vec::with_capacity(repeats);

        // warm-up
        let _ = if nproc == 1 {
            Command::new("./target/release/seq1").output()
        } else {
            Command::new("./target/release/parallel1")
                .env("RAYON_NUM_THREADS", nproc.to_string())
                .output()
        };

        for _ in 0..repeats {
            let output = if nproc == 1 {
                Command::new("./target/release/seq1").output()
            } else {
                Command::new("./target/release/parallel1")
                    .env("RAYON_NUM_THREADS", nproc.to_string())
                    .output()
            }
            .expect("failed to run");

            // parse numeric output (sim-time printed by the program)
            let val = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<f64>()
                .unwrap_or(f64::NAN);
            times.push(val);
        }

        let mean = times.iter().sum::<f64>() / times.len() as f64;
        let std_dev =
            (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();

        if nproc == 1 {
            seq_time = mean;
        }

        let speedup = seq_time / mean;
        let amdahl = 1.0 / (f_seq + (1.0 - f_seq) / nproc as f64);
        let ideal = nproc as f64;

        println!(
            "{nproc:>2} niti -> mean={mean:.4}s, speedup={speedup:.2}, amdahl={amdahl:.2}, ideal={ideal:.2}"
        );

        writeln!(
            file,
            "{},{:.4},{:.4},{:.4},{:.4},{:.4}",
            nproc, mean, std_dev, speedup, amdahl, ideal
        )
        .unwrap();
        file.flush().unwrap();
    }

    println!("\nRezultati su sačuvani u results_rust_strong.csv");
}
