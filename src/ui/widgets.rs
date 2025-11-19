use chrono::{Datelike, NaiveDate};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::calendar::conversion::ad_to_bs;
use super::theme::Theme;

pub fn render_keybindings(theme: &Theme) -> Paragraph<'static> {
    let lines = vec![
        Line::from(vec![
            Span::styled("Navigate: ", theme.key_style()),
            Span::raw("h/←"),
            Span::styled("/", theme.muted_style()),
            Span::raw("l/→ day"),
            Span::styled(" │ ", theme.muted_style()),
            Span::raw("k/↑"),
            Span::styled("/", theme.muted_style()),
            Span::raw("j/↓ week"),
            Span::styled(" │ ", theme.muted_style()),
            Span::raw("H/PgUp"),
            Span::styled("/", theme.muted_style()),
            Span::raw("L/PgDn month"),
            Span::styled(" │ ", theme.muted_style()),
            Span::raw("t/Home today"),
        ]),
        Line::from(vec![
            Span::styled("Input: ", theme.key_style()),
            Span::raw("a AD date"),
            Span::styled(" │ ", theme.muted_style()),
            Span::raw("b BS date"),
            Span::styled(" │ ", theme.muted_style()),
            Span::styled("Help: ", theme.key_style()),
            Span::raw("? help"),
            Span::styled(" │ ", theme.muted_style()),
            Span::styled("Quit: ", theme.key_style()),
            Span::raw("q/Esc"),
        ]),
    ];

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Keybindings"))
}

pub fn render_date_info<'a>(current_date: NaiveDate, error: Option<&'a String>, theme: &'a Theme) -> Paragraph<'a> {
    let mut lines = vec![];
    
    // Day of week
    let day_of_week = match current_date.weekday() {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    };
    
    // BS Date
    if let Ok(bs_date) = ad_to_bs(current_date) {
        lines.push(Line::from(vec![
            Span::styled("AD: ", theme.label_style()),
            Span::raw(format!("{}", current_date)),
            Span::styled(" → ", theme.muted_style()),
            Span::styled("BS: ", theme.label_style()),
            Span::raw(format!("{} ", bs_date.to_string())),
            Span::styled(format!("({})", day_of_week), theme.muted_style()),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::styled("AD: ", theme.label_style()),
            Span::raw(format!("{} ", current_date)),
            Span::styled(format!("({})", day_of_week), theme.muted_style()),
        ]));
    }
    
    lines.push(Line::from(""));
    
    // Today's date if different
    let today = chrono::Local::now().date_naive();
    if current_date != today {
        lines.push(Line::from(vec![
            Span::styled("Today:", theme.header_style()),
        ]));
        
        lines.push(Line::from(vec![
            Span::styled("AD: ", theme.label_style()),
            Span::raw(format!("{}", today)),
        ]));
        
        if let Ok(today_bs) = ad_to_bs(today) {
            lines.push(Line::from(vec![
                Span::styled("BS: ", theme.label_style()),
                Span::raw(today_bs.to_string()),
            ]));
        }
        
        lines.push(Line::from(""));
        
        // Days difference
        let diff = current_date.signed_duration_since(today).num_days();
        let diff_text = if diff > 0 {
            format!("{} days ahead", diff)
        } else {
            format!("{} days ago", -diff)
        };
        
        lines.push(Line::from(vec![
            Span::styled("Δ: ", theme.label_style()),
            Span::styled(diff_text, theme.muted_style()),
        ]));
    }
    
    // Error message if any
    if let Some(err) = error {
        lines.push(Line::from(""));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Error", theme.error_style()),
        ]));
        lines.push(Line::from(vec![
            Span::styled(err, theme.error_style()),
        ]));
    }
    
    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Date Info"))
        .wrap(Wrap { trim: true })
}


pub fn render_input_modal<'a>(
    input_mode: &'a str,
    input_buffer: &'a str,
    area: Rect,
    theme: &'a Theme,
) -> (Rect, Paragraph<'a>) {
    let modal_width = 50;
    let modal_height = 9;
    
    let modal_x = (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = (area.height.saturating_sub(modal_height)) / 2;
    
    let modal_area = Rect {
        x: modal_x,
        y: modal_y,
        width: modal_width,
        height: modal_height,
    };
    
    let title = match input_mode {
        "AD" => "Enter AD Date",
        "BS" => "Enter BS Date",
        _ => "Input",
    };
    
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Format: ", theme.label_style()),
            Span::styled("YYYY-MM-DD", theme.muted_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("> "),
            Span::styled(input_buffer, theme.title_style()),
            Span::styled("│", theme.title_style().add_modifier(ratatui::style::Modifier::SLOW_BLINK)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("Enter", theme.key_style()),
            Span::raw(" submit"),
            Span::styled(" │ ", theme.muted_style()),
            Span::styled("Esc", theme.key_style()),
            Span::raw(" cancel"),
        ]),
    ];
    
    let modal = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .style(theme.modal_style())
                .border_style(theme.modal_border_style()),
        );
    
    (modal_area, modal)
}

