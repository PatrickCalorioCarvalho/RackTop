use ratatui::{
    Frame,
    layout::{Rect, Constraint, Direction, Layout},
    widgets::{Block, Borders, Table, Row, Cell},
    style::{Style, Color, Modifier},
};

use crate::metrics::{Metrics, ProcessInfo};

pub fn draw(f: &mut Frame, area: Rect, metrics: &Metrics) {
    let outer_block = Block::default()
        .title(" Processes ")
        .borders(Borders::ALL);

    f.render_widget(outer_block.clone(), area);
    let inner_area = outer_block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(inner_area);

    let cpu_count = metrics.cpu_count as f32;

    // ==============================
    // 🔥 TOP CPU
    // ==============================
    let mut by_cpu: Vec<&ProcessInfo> = metrics.processes.iter().collect();

    by_cpu.sort_by(|a, b| {
        b.cpu
            .partial_cmp(&a.cpu)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let cpu_rows: Vec<Row> = by_cpu
        .into_iter()
        .take(20)
        .map(|p| {
            let normalized = p.cpu / cpu_count;

            let full_style = if p.cpu > 200.0 {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else if p.cpu > 100.0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            };

            Row::new(vec![
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.2}%", p.cpu)).style(full_style),
                Cell::from(format!("{:.2}%", normalized))
                    .style(Style::default().fg(Color::DarkGray)),
                Cell::from(format!("{} MB", p.memory / 1024)),
            ])
        })
        .collect();

    // ==============================
    // 🧠 TOP MEMÓRIA
    // ==============================
    let mut by_mem: Vec<&ProcessInfo> = metrics.processes.iter().collect();

    by_mem.sort_by(|a, b| b.memory.cmp(&a.memory));

    let mem_rows: Vec<Row> = by_mem
        .into_iter()
        .take(20)
        .map(|p| {
            let normalized = p.cpu / cpu_count;

            Row::new(vec![
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.2}%", p.cpu)),
                Cell::from(format!("{:.2}%", normalized)),
                Cell::from(format!("{} MB", p.memory / 1024))
                    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(7),       // PID
        Constraint::Percentage(35),  // Nome
        Constraint::Length(10),      // CPU Full
        Constraint::Length(10),      // CPU/Core
        Constraint::Length(12),      // Mem
    ];

    let cpu_table = Table::new(cpu_rows, widths)
        .header(
            Row::new(vec![
                Cell::from("PID"),
                Cell::from("Processo"),
                Cell::from("CPU 🔥")
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Cell::from("CPU/Core 🧮")
                    .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
                Cell::from("Mem"),
            ])
        )
        .block(
            Block::default()
                .title(" Top 20 CPU ")
                .borders(Borders::ALL),
        );

    let mem_table = Table::new(mem_rows, widths)
        .header(
            Row::new(vec![
                Cell::from("PID"),
                Cell::from("Processo"),
                Cell::from("CPU"),
                Cell::from("CPU/Core"),
                Cell::from("Mem 🧠")
                    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ])
        )
        .block(
            Block::default()
                .title(" Top 20 Memória ")
                .borders(Borders::ALL),
        );

    f.render_widget(cpu_table, chunks[0]);
    f.render_widget(mem_table, chunks[1]);
}