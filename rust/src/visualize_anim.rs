use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use gif::{Encoder, Frame, Repeat};
use std::fs::OpenOptions;
use plotters::style::RGBAColor;

#[derive(Debug)]
struct PendulumState {
    theta1: f64,
    theta2: f64,
    omega1: f64,
    omega2: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

fn read_trajectory(path: &str, y_offset: f64) -> Result<Vec<PendulumState>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    let l1 = 1.0;
    let l2 = 1.0;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 && line.contains("theta1") {
            continue;
        }
        let values: Vec<f64> = line
            .split(',')
            .map(|v| v.trim().parse::<f64>())
            .filter_map(Result::ok)
            .collect();

        if values.len() >= 6 {
            let theta1 = values[1];
            let omega1 = values[2];
            let theta2 = values[3];
            let omega2 = values[4];
            let x1 = l1 * theta1.sin();
            let y1 = -l1 * theta1.cos() + y_offset;
            let x2 = x1 + l2 * theta2.sin();
            let y2 = y1 - l2 * theta2.cos();
            data.push(PendulumState {theta1,theta2,omega1,omega2, x1, y1, x2, y2 });
        }
    }
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let trail_length = 30;
    let mut trail: Vec<(f64, f64)> = Vec::new();

    let y_offset = 0.5;
    let data = read_trajectory("rust_outputs/traj_000.csv", y_offset)?;
    let output_gif = "visualization/pendulum_animation.gif";

    let width = 800;
    let height = 600;
    let root_area = BitMapBackend::new("frame.png", (width, height)).into_drawing_area();

    // Pripremi GIF encoder
    let gif_file = OpenOptions::new().write(true).create(true).open(output_gif)?;
    let mut encoder = Encoder::new(gif_file, width as u16, height as u16, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    let m1 = 1.0;
    let m2 = 1.0;
    let l1 = 1.0;
    let l2 = 1.0;
    let g = 9.81;

    let frame_skip = 10;
    for (i, state) in data.iter().enumerate().step_by(frame_skip) {
        root_area.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root_area)
            .caption("Double pendulum", ("sans-serif", 24))
            .margin(20)
            .build_cartesian_2d(-2.0..2.0, -2.0..2.0)?;

        chart.configure_mesh().disable_mesh().draw()?;

        // Dodaj trenutnu poziciju druge mase u trag
        trail.push((state.x2, state.y2));
        if trail.len() > trail_length {
            trail.remove(0);
        }

        // Nacrtaj trag sa fade-out efektom
        for (j, &(tx, ty)) in trail.iter().enumerate() {
            let alpha = (j as f64) / (trail_length as f64); // 0.0 → 1.0
            let color = RGBAColor(255, 0, 0, alpha); // crveni sa providnošću
            chart.draw_series(PointSeries::of_element(
                vec![(tx, ty)],
                3,
                ShapeStyle::from(&color).filled(),
                &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st),
            ))?;
        }

        // fiksna gornja tacka (pivot)
        chart.draw_series(PointSeries::of_element(
            vec![(0.0, y_offset)],
            5,
            ShapeStyle::from(&BLACK).filled(),
            &|c, s, st| EmptyElement::at(c) + Circle::new((0,0), s, st),
        ))?;
        // Linije i tačke
        chart.draw_series(LineSeries::new(vec![(0.0, y_offset), (state.x1, state.y1)], &BLACK))?;
        chart.draw_series(LineSeries::new(vec![(state.x1, state.y1), (state.x2, state.y2)], &BLACK))?;
        chart.draw_series(PointSeries::of_element(
            vec![(state.x1, state.y1)],
            5,
            ShapeStyle::from(&BLUE).filled(),
            &|c, s, st| return EmptyElement::at(c) + Circle::new((0,0), s, st),
        ))?;
        chart.draw_series(PointSeries::of_element(
            vec![(state.x2, state.y2)],
            5,
            ShapeStyle::from(&RED).filled(),
            &|c, s, st| return EmptyElement::at(c) + Circle::new((0,0), s, st),
        ))?;

        // --- Izračunavanje energija ---
        let theta1 = state.theta1;
        let theta2 = state.theta2;
        let omega1 = state.omega1;
        let omega2 = state.omega2;

        let e_pot = - (m1 * g * l1 * theta1.cos() + m2 * g * (l1 * theta1.cos() + l2 * theta2.cos()));
        let e_kin = 0.5 * m1 * (l1 * omega1).powi(2)
            + 0.5 * m2
                * ((l1 * omega1).powi(2)
                    + (l2 * omega2).powi(2)
                    + 2.0 * l1 * l2 * omega1 * omega2 * (theta1 - theta2).cos());
        let e_tot = e_pot + e_kin;

        // --- Ispis energije na slici ---
        root_area.draw(&Text::new(
            format!(
                "E_pot = {:.2}  E_kin = {:.2}  E_tot = {:.2}",
                e_pot, e_kin, e_tot
            ),
            (50, 60),
            TextStyle::from(("sans-serif", 18).into_font()).color(&BLACK),
        ))?;

        // Pretvori trenutni frame u RGB bafer
        root_area.present()?;
        let frame_img = image::open("frame.png")?.to_rgb8();

        let mut frame = Frame::from_rgb(width as u16, height as u16, &frame_img);
        frame.delay = 2; // ~20 FPS (100/2 = 50ms po frejmu)
        encoder.write_frame(&frame)?;
    }

    println!("✅ GIF animacija sačuvana u {}", output_gif);
    Ok(())
}
