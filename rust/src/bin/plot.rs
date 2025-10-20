use csv::ReaderBuilder;
use plotters::prelude::*;
use std::error::Error;

#[derive(Debug)]
struct DataPoint {
    nproc: f64,
    mean: f64,
    std_dev: f64,
    speedup: f64,
    theory: f64,
    ideal: f64,
}

fn read_csv(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let rec = result?;
        let nproc: f64 = rec[0].parse()?;
        let mean: f64 = rec[1].parse()?;
        let std_dev: f64 = rec[2].parse()?;
        let speedup: f64 = rec[3].parse()?;
        let theory: f64 = rec[4].parse()?;
        let ideal: f64 = rec[5].parse()?;
        data.push(DataPoint { nproc, mean, std_dev, speedup, theory, ideal });
    }
    Ok(data)
}

fn plot_scaling(data: &[DataPoint], title: &str, theory_label: &str, output: &str)
    -> Result<(), Box<dyn Error>>
{
    let root = BitMapBackend::new(output, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = data.iter().map(|d| d.ideal).fold(0.0, f64::max) * 1.1;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 25))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(1f64..data.last().unwrap().nproc, 0f64..max_y)?;

    chart.configure_mesh()
        .x_desc("Broj niti")
        .y_desc("Ubrzanje (Speedup)")
        .light_line_style(&WHITE.mix(0.8))
        .draw()?;

    // Eksperimentalni podaci (plavo)
    chart.draw_series(LineSeries::new(
        data.iter().map(|d| (d.nproc, d.speedup)),
        &BLUE,
    ))?.label("Eksperimentalni podaci")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Teorijski (crveno)
    chart.draw_series(LineSeries::new(
        data.iter().map(|d| (d.nproc, d.theory)),
        &RED,
    ))?.label(theory_label)
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Idealno (crno)
    chart.draw_series(LineSeries::new(
        data.iter().map(|d| (d.nproc, d.ideal)),
        &BLACK,
    ))?.label("Idealno skaliranje")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.9))
        .draw()?;

    root.present()?;
    println!("✅ Sačuvan grafikon: {output}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Plotting Rust Scaling Results ---");

    // Jako skaliranje
    if let Ok(data_strong) = read_csv("results_rust_strong.csv") {
        plot_scaling(
            &data_strong,
            "Jako skaliranje (Rust)",
            "Amdalov zakon",
            "results_rust_strong.png"
        )?;
    } else {
        println!("⚠️ Nije pronađen results_rust_strong.csv");
    }

    // Slabo skaliranje
    if let Ok(data_weak) = read_csv("results_rust_weak.csv") {
        plot_scaling(
            &data_weak,
            "Slabo skaliranje (Rust)",
            "Gustafsonov zakon",
            "results_rust_weak.png"
        )?;
    } else {
        println!("⚠️ Nije pronađen results_rust_weak.csv");
    }

    Ok(())
}
