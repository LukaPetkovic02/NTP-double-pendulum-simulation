use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct PendulumState {
    t: f64,
    theta1: f64,
    theta2: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

fn read_trajectory(path: &str) -> Result<Vec<PendulumState>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    let l1 = 1.0;
    let l2 = 1.0;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 && line.contains("theta1") {
            continue; // preskoči zaglavlje
        }
        let values: Vec<f64> = line
            .split(',')
            .map(|v| v.trim().parse::<f64>())
            .filter_map(Result::ok)
            .collect();
        if values.len() >= 6 {
            let (t, theta1, _omega1, theta2, _omega2, _energy) =
                (values[0], values[1], values[2], values[3], values[4], values[5]);
            let x1 = l1 * theta1.sin();
            let y1 = -l1 * theta1.cos();
            let x2 = x1 + l2 * theta2.sin();
            let y2 = y1 - l2 * theta2.cos();
            data.push(PendulumState {
                t,
                theta1,
                theta2,
                x1,
                y1,
                x2,
                y2,
            });
        }
    }
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_trajectory("rust_outputs/traj_000.csv")?;

    let root = BitMapBackend::new("rust_outputs/pendulum_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Dvostruko klatno – trajektorija", ("sans-serif", 24))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-2.0..2.0, -2.0..2.0)?;

    chart.configure_mesh().x_desc("x").y_desc("y").draw()?;

    // Putanja prve mase (kraja prve šipke)
    chart
        .draw_series(LineSeries::new(
            data.iter().map(|s| (s.x1, s.y1)),
            &BLUE,
        ))?
        .label("Masa 1")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Putanja druge mase (kraja druge šipke)
    chart
        .draw_series(LineSeries::new(
            data.iter().map(|s| (s.x2, s.y2)),
            &RED,
        ))?
        .label("Masa 2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    println!("✅ Slika sačuvana u rust_outputs/pendulum_plot.png");
    Ok(())
}
