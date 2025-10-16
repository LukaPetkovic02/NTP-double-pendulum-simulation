use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use gif::{Encoder, Frame, Repeat};
use std::fs::OpenOptions;

#[derive(Debug)]
struct PendulumState {
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
            let theta2 = values[3];
            let x1 = l1 * theta1.sin();
            let y1 = -l1 * theta1.cos() + y_offset;
            let x2 = x1 + l2 * theta2.sin();
            let y2 = y1 - l2 * theta2.cos();
            data.push(PendulumState { x1, y1, x2, y2 });
        }
    }
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
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

    let frame_skip = 10;
    for (i, state) in data.iter().enumerate().step_by(frame_skip) {
        root_area.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root_area)
            .caption("Double pendulum", ("sans-serif", 24))
            .margin(20)
            .build_cartesian_2d(-2.0..2.0, -2.0..2.0)?;

        chart.configure_mesh().disable_mesh().draw()?;

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
