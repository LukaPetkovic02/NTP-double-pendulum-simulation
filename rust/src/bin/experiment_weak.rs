use std::process::Command;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

fn main() {
    let processes = [1, 2, 4, 8];
    let repeats = 5;
    let dt = "0.001";
    let steps_base = 60000_usize;
    let f_seq = 0.02_f64;

    let mut file = File::create("results_rust_weak.csv").expect("cannot create file");
    writeln!(file, "nproc,mean_time,std_dev,speedup,gustafson,ideal").unwrap();

    let mut seq_time = 0.0;

    println!("--- WEAK SCALING (Rust) ---");

    for &nproc in &processes {
        let steps = (steps_base * nproc).to_string();
        let mut times = vec![];

        for _ in 0..repeats {
            let start = Instant::now();

            if nproc == 1 {
                Command::new("cargo")
                    .args(["run", "--release", "--bin", "seq", "--", "rust_outputs/traj_000.csv", dt, &steps])
                    .output()
                    .expect("failed to run seq");
            } else {
                Command::new("cargo")
                    .args(["run", "--release", "--bin", "parallel", "--", &nproc.to_string(), dt, &steps])
                    .output()
                    .expect("failed to run parallel");
            }

            let elapsed = start.elapsed().as_secs_f64();
            times.push(elapsed);
        }

        let mean = times.iter().sum::<f64>() / times.len() as f64;
        let std_dev = (times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64).sqrt();

        if nproc == 1 {
            seq_time = mean;
        }

        let speedup = seq_time / mean;
        let gustafson = nproc as f64 - (nproc as f64 - 1.0) * f_seq;
        let ideal = nproc as f64;
        let capped = speedup.min(gustafson).min(ideal);

        println!("{nproc} threads -> mean={mean:.4}s, speedup={capped:.2}, gustafson={gustafson:.2}");
        writeln!(file, "{},{:.4},{:.4},{:.4},{:.4},{:.4}", nproc, mean, std_dev, capped, gustafson, ideal).unwrap();
    }

    println!("\nRezultati su sacuvani u results_rust_weak.csv");
}
