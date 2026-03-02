pub mod layout;
pub mod tab_system;
pub mod tab_processes;
pub mod tab_docker;

use ratatui::Frame;
use crate::{metrics::Metrics, app::Tab};

pub fn draw_ui(f: &mut Frame, metrics: &Metrics, title: &str, tab: Tab) {
    layout::draw_layout(f, metrics, title, tab);
}