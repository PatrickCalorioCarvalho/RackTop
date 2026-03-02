use ratatui::{
    Frame,
    layout::{Rect, Layout, Direction, Constraint},
    widgets::{Block, Borders, Gauge},
    style::{Style, Color},
};

use crate::metrics::Metrics;

pub fn draw(f: &mut Frame, area: Rect, metrics: &Metrics) {
    let outer = Block::default()
        .title("System")
        .borders(Borders::ALL);

    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let mut constraints = vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ];

    // adiciona espaço para cada disco
    for _ in &metrics.disks {
        constraints.push(Constraint::Length(3));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    // ===== CPU =====
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(metrics.cpu_usage as f64 / 100.0)
        .label(format!("{:.2}%", metrics.cpu_usage));

    f.render_widget(cpu_gauge, chunks[0]);

    // ===== MEM =====
    let mem_ratio =
        metrics.memory_used as f64 / metrics.memory_total as f64;

    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memória").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(mem_ratio)
        .label(format!(
            "{} / {} MB",
            metrics.memory_used / 1024,
            metrics.memory_total / 1024
        ));

    f.render_widget(mem_gauge, chunks[1]);

    // ===== SWAP =====
    let swap_ratio =
        if metrics.swap_total > 0 {
            metrics.swap_used as f64 / metrics.swap_total as f64
        } else {
            0.0
        };

    let swap_gauge = Gauge::default()
        .block(Block::default().title("Swap").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(swap_ratio)
        .label(format!(
            "{} / {} MB",
            metrics.swap_used / 1024,
            metrics.swap_total / 1024
        ));

    f.render_widget(swap_gauge, chunks[2]);

    // ===== DISKS =====
    for (i, disk) in metrics.disks.iter().enumerate() {
        let ratio = disk.used as f64 / disk.total as f64;

        let disk_gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!("Disk {}", disk.name))
                    .borders(Borders::ALL),
            )
            .gauge_style(Style::default().fg(Color::Magenta))
            .ratio(ratio)
            .label(format!(
                "{:.1} GB / {:.1} GB",
                disk.used as f64 / 1_000_000_000.0,
                disk.total as f64 / 1_000_000_000.0
            ));

        f.render_widget(disk_gauge, chunks[3 + i]);
    }
}