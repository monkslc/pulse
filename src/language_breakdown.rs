use plotters::prelude::*;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::error::Error;

type LanguageBreakdown = HashMap<String, u32>;

pub fn get_language_breakdown(
    db: &Connection,
    start_time: u32,
    end_time: u32,
) -> Result<LanguageBreakdown, Box<dyn std::error::Error>> {
    let mut statement = db.prepare(
        "
            SELECT language, COUNT(language) as count
            FROM pulses
            WHERE time > ?1 AND time < ?2
            GROUP BY language
            ",
    )?;

    let mut breakdown = HashMap::new();
    let mut rows = statement.query(params![start_time, end_time])?;
    while let Ok(Some(row)) = rows.next() {
        let count: u32 = row.get_unwrap(1);
        let language: String = row.get_unwrap(0);
        breakdown.insert(language, count);
    }
    Ok(breakdown)
}

pub fn create_language_breakdown_chart(
    lb: LanguageBreakdown,
    output_file: String,
) -> Result<(), Box<dyn Error>> {
    let (language_count, pulse_count) =
        lb.values()
            .fold((0, 0), |(total_languages, total_pulses), pulse_count| {
                return (total_languages + 1, total_pulses + pulse_count);
            });

    let mut language_percentages: Vec<(&String, u32)> = lb
        .iter()
        .map(|(language, count)| (language, count * 100 / pulse_count))
        .collect();

    language_percentages.sort_by(|a, b| a.1.cmp(&b.1));

    let root = BitMapBackend::new(&output_file, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Language Breakdown", ("sans-serif", 50.0).into_font())
        .build_ranged(0u32..100u32, 0u32..language_count)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("language")
        .x_desc("% of activity")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;

    chart.draw_series(
        Histogram::horizontal(&chart)
            .style(RED.mix(0.5).filled())
            .data(
                language_percentages
                    .iter()
                    .enumerate()
                    .map(|(i, lc)| (i as u32, lc.1)),
            ),
    )?;
    Ok(())
}
