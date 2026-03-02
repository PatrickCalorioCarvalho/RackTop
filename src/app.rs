use std::{
    io::stdout,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    cursor::{Hide, Show},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::metrics::Metrics;
use crate::ui::draw_ui;

#[derive(Clone, Copy)]
pub enum Tab {
    System,
    Processes,
    Docker,
}

pub fn run(title: String) {
    enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();

    let mut current_tab = Tab::System;

    loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,

                    KeyCode::Right => {
                        current_tab = match current_tab {
                            Tab::System => Tab::Processes,
                            Tab::Processes => Tab::Docker,
                            Tab::Docker => Tab::System,
                        }
                    }

                    KeyCode::Left => {
                        current_tab = match current_tab {
                            Tab::System => Tab::Docker,
                            Tab::Processes => Tab::System,
                            Tab::Docker => Tab::Processes,
                        }
                    }

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let metrics = Metrics::collect();

            terminal.draw(|f| {
                draw_ui(f, &metrics, &title, current_tab);
            }).unwrap();

            last_tick = Instant::now();
        }
    }

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        Show
    ).unwrap();
}