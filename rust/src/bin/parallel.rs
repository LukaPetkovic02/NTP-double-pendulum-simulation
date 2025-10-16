use std::thread;
use std::sync::Arc;
use std::path::Path;
use std::env;
use std::time::Instant;
use num_cpus;

mod common {
    // copy/paste derivatives, energy, rk4, integrate into a module to reuse
    use csv::Writer;
    use std::path::Path;
    use std::time::Instant;

    #[derive(Clone, Copy)]
    pub struct Params {
        pub m1: f64,
        pub m2: f64,
        pub l1: f64,
        pub l2: f64,
        pub g: f64,
    }

    pub fn derivatives(state: &[f64;4], p: &Params) -> [f64;4] {
        let theta1 = state[0];
        let omega1 = state[1];
        let theta2 = state[2];
        let omega2 = state[3];

        let m1 = p.m1;
        let m2 = p.m2;
        let l1 = p.l1;
        let l2 = p.l2;
        let g  = p.g;

        let delta = theta2 - theta1;
        let sin_d = delta.sin();
        let cos_d = delta.cos();
        let denom = m1 + m2 * sin_d * sin_d;

        let dtheta1 = omega1;
        let dtheta2 = omega2;

        let domega1 = (
            m2 * l1 * omega1 * omega1 * sin_d * cos_d
            + m2 * g * theta2.sin() * cos_d
            + m2 * l2 * omega2 * omega2 * sin_d
            - (m1 + m2) * g * theta1.sin()
        ) / (l1 * denom);

        let domega2 = (
            - m2 * l2 * omega2 * omega2 * sin_d * cos_d
            + (m1 + m2) * (g * theta1.sin() * cos_d - l1 * omega1 * omega1 * sin_d - g * theta2.sin())
        ) / (l2 * denom);

        [dtheta1, domega1, dtheta2, domega2]
    }

    pub fn energy(state: &[f64;4], p: &Params) -> f64 {
        let theta1 = state[0];
        let omega1 = state[1];
        let theta2 = state[2];
        let omega2 = state[3];

        let m1 = p.m1;
        let m2 = p.m2;
        let l1 = p.l1;
        let l2 = p.l2;
        let g  = p.g;

        let x1 = l1 * theta1.sin();
        let y1 = -l1 * theta1.cos();
        let _x2 = x1 + l2 * theta2.sin();
        let y2 = y1 - l2 * theta2.cos();

        let v1x = l1 * omega1 * theta1.cos();
        let v1y = l1 * omega1 * theta1.sin();
        let v2x = v1x + l2 * omega2 * theta2.cos();
        let v2y = v1y + l2 * omega2 * theta2.sin();

        let t = 0.5 * m1 * (v1x*v1x + v1y*v1y) + 0.5 * m2 * (v2x*v2x + v2y*v2y);
        let v = m1 * g * y1 + m2 * g * y2;
        t + v
    }

    pub fn rk4_step(y: &[f64;4], dt: f64, p: &Params) -> [f64;4] {
        let k1 = derivatives(y, p);
        let y2 = [
            y[0] + 0.5*dt*k1[0],
            y[1] + 0.5*dt*k1[1],
            y[2] + 0.5*dt*k1[2],
            y[3] + 0.5*dt*k1[3],
        ];
        let k2 = derivatives(&y2, p);
        let y3 = [
            y[0] + 0.5*dt*k2[0],
            y[1] + 0.5*dt*k2[1],
            y[2] + 0.5*dt*k2[2],
            y[3] + 0.5*dt*k2[3],
        ];
        let k3 = derivatives(&y3, p);
        let y4 = [
            y[0] + dt*k3[0],
            y[1] + dt*k3[1],
            y[2] + dt*k3[2],
            y[3] + dt*k3[3],
        ];
        let k4 = derivatives(&y4, p);

        [
            y[0] + (dt/6.0)*(k1[0] + 2.0*k2[0] + 2.0*k3[0] + k4[0]),
            y[1] + (dt/6.0)*(k1[1] + 2.0*k2[1] + 2.0*k3[1] + k4[1]),
            y[2] + (dt/6.0)*(k1[2] + 2.0*k2[2] + 2.0*k3[2] + k4[2]),
            y[3] + (dt/6.0)*(k1[3] + 2.0*k2[3] + 2.0*k3[3] + k4[3])
        ]
    }

    pub fn integrate(mut y: [f64;4], dt: f64, steps: usize, p: &Params, out_path: &Path) -> std::io::Result<f64> {
        let mut wtr = Writer::from_path(out_path)?;
        wtr.write_record(&["t","theta1","omega1","theta2","omega2","energy"])?;

        let startt = Instant::now();
        wtr.write_record(&["0.0",
            &format!("{}", y[0]),
            &format!("{}", y[1]),
            &format!("{}", y[2]),
            &format!("{}", y[3]),
            &format!("{}", energy(&y, p))
        ])?;

        let mut t = 0.0_f64;
        for i in 1..=steps {
            y = rk4_step(&y, dt, p);
            t = i as f64 * dt;
            wtr.write_record(&[
                &format!("{}", t),
                &format!("{}", y[0]),
                &format!("{}", y[1]),
                &format!("{}", y[2]),
                &format!("{}", y[3]),
                &format!("{}", energy(&y, p))
            ])?;
        }
        wtr.flush()?;
        let elapsed = startt.elapsed().as_secs_f64();
        Ok(elapsed)
    }
}

use common::{Params};

fn main() {
    let args: Vec<String> = env::args().collect();
    // usage: parallel <num_runs> <dt> <steps>
    let num_runs: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(num_cpus::get());
    let dt: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.001);
    let steps: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(60000);

    let params = Params { m1:1.0, m2:1.0, l1:1.0, l2:1.0, g:9.81 };
    let base = [std::f64::consts::FRAC_PI_2, 0.0, std::f64::consts::FRAC_PI_2 + 0.01, 0.0];
    let out_dir = Path::new("rust_outputs/parallel");
    std::fs::create_dir_all(out_dir).unwrap();

    let base_arc = Arc::new(params);

    let mut handles = Vec::new();
    let start_all = Instant::now();
    for i in 0..num_runs {
        let p = *base_arc;
        let dt = dt;
        let steps = steps;
        let out_path = out_dir.join(format!("traj_{:03}.csv", i));
        // make initial condition slightly different
        let mut y0 = base;
        y0[2] += (i as f64 - (num_runs as f64)/2.0) * 1e-3;
        let handle = thread::spawn(move || {
            let t = common::integrate(y0, dt, steps, &p, &out_path).expect("integrate failed");
            (i, t)
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for h in handles {
        let r = h.join().unwrap();
        results.push(r);
    }
    let elapsed = start_all.elapsed().as_secs_f64();
    println!("Per-sim times: {:?}", results);
    println!("Total parallel wall-clock: {:.4}s", elapsed);
}
