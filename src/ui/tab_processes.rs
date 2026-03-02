use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, area: Rect) {
    let body = Paragraph::new("Top processos (em breve)")
        .block(Block::default().title("Processes").borders(Borders::ALL));

    f.render_widget(body, area);
}