use std::process::Command;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

fn main() {
    let processes = [1, 2, 4, 8];
    let repeats = 30;
    let f_seq = 0.05_f64;

    let mut file = File::create("results_rust_strong.csv").expect("cannot create file");
    writeln!(file, "nproc,mean_time,std_dev,speedup,amdahl,ideal").unwrap();

    let mut seq_time = 0.0;

    println!("--- STRONG SCALING (Rust Optimized) ---");

    for &nproc in &processes {
        let mut times = vec![];
        for _ in 0..repeats {
            let start = Instant::now();

            if nproc == 1 {
                Command::new("./target/release/seq1")
                    .output()
                    .expect("failed to run seq1");
            } else {
                Command::new("./target/release/parallel1")
                    .env("RAYON_NUM_THREADS", nproc.to_string())
                    .output()
                    .expect("failed to run parallel1");
            }

            let elapsed = start.elapsed().as_secs_f64();
            times.push(elapsed);
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
        let capped = speedup.min(amdahl).min(ideal);

        println!(
            "{nproc} threads -> mean={mean:.4}s, speedup={capped:.2}, amdahl={amdahl:.2}"
        );
        writeln!(
            file,
            "{},{:.4},{:.4},{:.4},{:.4},{:.4}",
            nproc, mean, std_dev, capped, amdahl, ideal
        )
        .unwrap();
    }

    println!("\nRezultati su sačuvani u results_rust_strong.csv");
}
