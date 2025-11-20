use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub primary: Color,
    pub accent: Color,
    pub success: Color,
    pub alert: Color,
    pub muted: Color,
    pub text: Color,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            primary: Color::Cyan,
            accent: Color::Yellow,
            success: Color::Green,
            alert: Color::Red,
            muted: Color::Gray,
            text: Color::White,
        }
    }

    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(Color::Black)
            .bg(self.success)
            .add_modifier(Modifier::BOLD)
    }

    pub fn today_style(&self) -> Style {
        Style::default()
            .fg(Color::White)
            .bg(self.alert)
            .add_modifier(Modifier::BOLD)
    }

    pub fn label_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
    }

    pub fn error_style(&self) -> Style {
        Style::default()
            .fg(self.alert)
            .add_modifier(Modifier::BOLD)
    }

    pub fn muted_style(&self) -> Style {
        Style::default()
            .fg(self.muted)
    }

    pub fn key_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn modal_border_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
    }
}
