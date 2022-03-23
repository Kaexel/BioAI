use crate::parsing::TrainData;
use plotters::prelude::*;
pub fn plot_train_file(data: &TrainData) {
    let m: Vec<(i32, i32)> = data.patients.iter().map(|(a, b)| (b.x_coord, b.y_coord)).collect();
    let root_area = BitMapBackend::new("test.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let max_y = m.iter().map(|(x,y)|y).max().expect("NISSE");
    let max_x = m.iter().map(|(x,y)|x).max().expect("NISSE");
    let min_y = m.iter().map(|(x,y)|y).min();
    let min_x = m.iter().map(|(x,y)|x).min();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("Patients", ("sans-serif", 40.0))
        .build_cartesian_2d(0..*max_x, 0..*max_y)
        .unwrap();

    // Draw Scatter Plot
    ctx.draw_series(
        m.iter().map(|point| Circle::new(*point, 4i32, &BLUE)),
    ).unwrap();
    println!("{:?}", max_x);
    println!("{:?}", max_y);
}