use ratatui::{
    Frame,
    layout::{Rect, Constraint},
    widgets::{Block, Borders, Table, Row, Cell},
    style::{Style, Color},
};

use crate::docker;

pub fn draw(f: &mut Frame, area: Rect) {
    let containers = docker::list_containers();

    let header = Row::new(vec![
        Cell::from("ID"),
        Cell::from("Name"),
        Cell::from("Status"),
        Cell::from("CPU%"),
        Cell::from("Mem%"),
    ])
    .style(Style::default().fg(Color::Yellow));

    let rows: Vec<Row> = containers
        .iter()
        .map(|c| {
            let status_color = if c.status.contains("Up") {
                Color::Green
            } else {
                Color::Red
            };

            Row::new(vec![
                Cell::from(c.id.clone()),
                Cell::from(c.name.clone()),
                Cell::from(c.status.clone()).style(Style::default().fg(status_color)),
                Cell::from(c.cpu.clone()),
                Cell::from(c.mem.clone()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(12),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(10),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().title("Docker").borders(Borders::ALL));

    f.render_widget(table, area);
}