use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::theme::Theme;

pub fn render_help_screen(theme: &Theme) -> Paragraph<'static> {
    let lines = vec![
        Line::from(vec![
            Span::styled("Miti - Calendar Viewer", theme.title_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("NAVIGATION", theme.header_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  h / ←", theme.key_style()),
            Span::raw("  Move backward by 1 day"),
        ]),
        Line::from(vec![
            Span::styled("  l / →", theme.key_style()),
            Span::raw("  Move forward by 1 day"),
        ]),
        Line::from(vec![
            Span::styled("  k / ↑", theme.key_style()),
            Span::raw("  Move backward by 1 week"),
        ]),
        Line::from(vec![
            Span::styled("  j / ↓", theme.key_style()),
            Span::raw("  Move forward by 1 week"),
        ]),
        Line::from(vec![
            Span::styled("  H / PgUp", theme.key_style()),
            Span::raw("  Move backward by 1 month"),
        ]),
        Line::from(vec![
            Span::styled("  L / PgDn", theme.key_style()),
            Span::raw("  Move forward by 1 month"),
        ]),
        Line::from(vec![
            Span::styled("  t / Home", theme.key_style()),
            Span::raw("  Jump to today's date"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("DATE INPUT", theme.header_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  a", theme.key_style()),
            Span::raw("  Enter AD date (format: YYYY-MM-DD)"),
        ]),
        Line::from(vec![
            Span::styled("  b", theme.key_style()),
            Span::raw("  Enter BS date (format: YYYY-MM-DD)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("OTHER", theme.header_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ?", theme.key_style()),
            Span::raw("  Show this help screen"),
        ]),
        Line::from(vec![
            Span::styled("  q / Esc", theme.key_style()),
            Span::raw("  Quit application"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("CALENDAR LEGEND", theme.header_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Red", theme.today_style()),
            Span::raw("    Today's date"),
        ]),
        Line::from(vec![
            Span::styled("  Green", theme.selected_style()),
            Span::raw("  Currently selected date"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("ABOUT", theme.header_style()),
        ]),
        Line::from(""),
        Line::from("  Miti - Dual calendar viewer (AD ↔ BS)"),
        Line::from("  Version: 0.1.0"),
        Line::from("  Supported range: 2000-2100 BS"),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Press any key to close", theme.muted_style()),
        ]),
    ];

    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(theme.modal_border_style()),
        )
        .wrap(Wrap { trim: false })
}

pub fn get_help_modal_area(area: Rect) -> Rect {
    let width = 60.min(area.width.saturating_sub(4));
    let height = 45.min(area.height.saturating_sub(4));
    
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    
    Rect {
        x,
        y,
        width,
        height,
    }
}
