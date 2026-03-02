use ratatui::{
    Frame,
    layout::{Rect, Constraint},
    widgets::{Block, Borders, Table, Row, Cell},
    style::{Style, Color, Modifier},
};

use crate::docker;

fn progress_bar(percent: f64, width: usize) -> String {
    let filled = ((percent / 100.0) * width as f64).round() as usize;
    let empty = width - filled;

    format!(
        "{}{} {:>5.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percent
    )
}

pub fn draw(f: &mut Frame, area: Rect) {
    let outer = Block::default()
        .title("Docker Containers")
        .borders(Borders::ALL);

    f.render_widget(outer.clone(), area);
    let inner = outer.inner(area);

    let containers = docker::list_containers();

    let header = Row::new(vec![
        Cell::from("Name").style(Style::default().fg(Color::Yellow)),
        Cell::from("Status").style(Style::default().fg(Color::Yellow)),
        Cell::from("CPU").style(Style::default().fg(Color::Yellow)),
        Cell::from("Memory").style(Style::default().fg(Color::Yellow)),
    ]);

    let rows: Vec<Row> = containers
        .iter()
        .map(|c| {
            let status_color = if c.status.contains("Up") {
                Color::Green
            } else {
                Color::Red
            };

            // converter string para número
            let cpu_value: f64 = c.cpu.replace('%', "").parse().unwrap_or(0.0);
            let mem_value: f64 = c.mem.replace('%', "").parse().unwrap_or(0.0);

            let cpu_bar = progress_bar(cpu_value, 10);
            let mem_bar = progress_bar(mem_value, 10);

            let cpu_color = if cpu_value > 70.0 {
                Color::Red
            } else if cpu_value > 40.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            Row::new(vec![
                Cell::from(c.name.clone())
                    .style(Style::default().add_modifier(Modifier::BOLD)),
                Cell::from(c.status.clone())
                    .style(Style::default().fg(status_color)),
                Cell::from(cpu_bar).style(Style::default().fg(cpu_color)),
                Cell::from(mem_bar).style(Style::default().fg(Color::Cyan)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(table, inner);
}