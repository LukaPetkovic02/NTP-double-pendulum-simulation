use std::process::Command;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

fn main() {
    let processes = [1,2,4,8,16];
    let repeats = 30;
    let f_seq = 0.05_f64;
    let base_runs = 2;
    let steps = 600_000;    

    let mut file = File::create("results_rust_weak.csv").unwrap();
    writeln!(file, "nproc,mean_time,std_dev,scaled_speedup,gustafson,ideal").unwrap();

    let mut t1 = 0.0;
    println!("--- WEAK SCALING (Rust, CPU-only) ---");
    for &p in &processes {
        let runs = p * base_runs;  // posao ∝ p
        let mut times = vec![];
        for _ in 0..repeats {
            let start = Instant::now();
            Command::new("./target/release/parallel1")
                .env("RAYON_NUM_THREADS", p.to_string())
                .args([format!("--runs={runs}"), format!("--steps={steps}")])
                .output().unwrap();
            times.push(start.elapsed().as_secs_f64());
        }
        let mean = times.iter().sum::<f64>()/times.len() as f64;
        let std  = (times.iter().map(|t|(t-mean).powi(2)).sum::<f64>()/times.len() as f64).sqrt();

        if p==1 { t1 = mean; }
        let scaled_speedup = (p as f64) * (t1/mean);
        let gustafson = p as f64 - (p as f64 - 1.0)*f_seq;
        let ideal = p as f64;

        println!("{p} threads → mean={mean:.4}s, S_scaled={scaled_speedup:.2}, Gustafson={gustafson:.2}");
        writeln!(file, "{},{:.6},{:.6},{:.4},{:.4},{:.4}", p, mean, std, scaled_speedup, gustafson, ideal).unwrap();
    }
    println!("Sačuvano u results_rust_weak.csv");
}
