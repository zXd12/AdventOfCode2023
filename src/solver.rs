use plotters::prelude::*;
use std::{
    fs::File,
    io::{self},
    time::{Duration, Instant},
};

use crate::questions::*;

pub fn solve(day: u32, question: u32, path: Option<&str>) -> u32 {
    let default_path = &format!("inputs/input{}.txt", day);
    let input_path = path.unwrap_or_else(|| default_path);
    let file = File::open(input_path).unwrap();
    let input = io::BufReader::new(file);
    return get_function(day, question)(input).unwrap();
}

pub fn perf_test(day: u32, question: u32, path: Option<&str>, iterations: u32) -> u32 {
    let default_path = &format!("inputs/input{}.txt", day);
    let input_path = path.unwrap_or_else(|| default_path);
    let file = File::open(input_path).unwrap();
    let fun = get_function(day, question);
    let mut total_time = Duration::new(0, 0);
    let mut min_time = u32::MAX;
    let mut max_time = u32::MIN;
    let mut data: Vec<u32> = vec![];

    // warmup
    let reader = io::BufReader::new(file.try_clone().unwrap());
    let _ = fun(reader);

    for _ in 0..iterations {
        let reader = io::BufReader::new(file.try_clone().unwrap());
        let start_time = Instant::now();
        let _ = fun(reader);
        let execution_time = Instant::now() - start_time;
        total_time += execution_time;
        let nanos = execution_time.subsec_nanos();
        data.push(nanos);
        min_time = min_time.min(nanos);
        max_time = max_time.max(nanos);
    }

    let avg = u32::try_from(total_time.as_nanos() / iterations as u128).unwrap();

    // Create a drawing area
    let file_name = format!("perfs/day{}_question{}_plot.png", day, question);
    let root = BitMapBackend::new(&file_name, (1600, 1200)).into_drawing_area();

    // Configure the chart with a logarithmic y-axis
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "Day {} Question {} execution time ({:e} iterations)",
                day, question, iterations
            ),
            ("Arial", 80).into_font(),
        )
        .margin(20)
        .y_label_area_size(100)
        .build_cartesian_2d(0u32..(data.len() as u32), (min_time..max_time).log_scale())
        .unwrap();

    // Configure mesh to automatically fit the graph height to the data
    chart
        .configure_mesh()
        .axis_desc_style(("Arial", 40).into_font())
        .y_desc("Time (nanosecond)")
        .label_style(("Arial", 40).into_font())
        .y_label_formatter(&|x| format!("{:e}", x))
        .disable_x_mesh()
        .disable_x_axis()
        .draw()
        .unwrap();

    // Draw the line series
    let _ = chart.draw_series(
        data.iter()
            .enumerate()
            .map(|(x, &y)| Circle::new((x as u32, y), 3, BLACK.filled())),
    );

    let _ = chart.draw_series(LineSeries::new(
        vec![(0, avg), (iterations - 2, avg)],
        RED.stroke_width(7),
    )).unwrap().label(format!("avg : {}ns", avg)).legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], RED.filled()));

    chart.configure_series_labels().label_font(("Arial", 40).into_font()).border_style(BLACK).background_style(WHITE.filled()).position(SeriesLabelPosition::UpperRight).draw().unwrap();

    // Save the plot to a file
    root.present().unwrap();

    avg
}

fn get_function(day: u32, question: u32) -> fn(io::BufReader<File>) -> Result<u32, io::Error> {
    match (day, question) {
        (1, 1) => day1::part_one,
        (1, 2) => day1::part_two,
        (2, 1) => day2::part_one,
        (2, 2) => day2::part_two,
        (3, 1) => day3::part_one,
        (3, 2) => day3::part_two,
        _ => panic!("Unsuported day or question!"),
    }
}
