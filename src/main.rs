mod calendar;
mod ui;

use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;

use calendar::conversion::{ad_to_bs, bs_to_ad, NepaliDate};
use calendar::bs_data::get_days_in_month;
use ui::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
enum InputMode {
    Normal,
    EnteringAD,
    EnteringBS,
    Help,
}

struct App {
    current_date_ad: NaiveDate,
    input_mode: InputMode,
    input_buffer: String,
    error_message: Option<String>,
    theme: Theme,
}

impl App {
    fn new() -> Self {
        Self {
            current_date_ad: Local::now().date_naive(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            error_message: None,
            theme: Theme::default(),
        }
    }

    fn move_day(&mut self, days: i64) {
        if let Some(new_date) = self.current_date_ad.checked_add_signed(chrono::Duration::days(days)) {
            self.current_date_ad = new_date;
            self.error_message = None;
        }
    }

    fn move_week(&mut self, weeks: i64) {
        self.move_day(weeks * 7);
    }

    fn move_month(&mut self, months: i32) {
        if let Some(new_date) = if months > 0 {
            self.current_date_ad.checked_add_months(chrono::Months::new(months as u32))
        } else {
            self.current_date_ad.checked_sub_months(chrono::Months::new((-months) as u32))
        } {
            self.current_date_ad = new_date;
            self.error_message = None;
        }
    }

    fn jump_to_today(&mut self) {
        self.current_date_ad = Local::now().date_naive();
        self.error_message = None;
    }

    fn handle_input_submit(&mut self) {
        let input = self.input_buffer.trim();
        
        match self.input_mode {
            InputMode::EnteringAD => {
                let parts: Vec<&str> = input.split(|c| c == '-' || c == '/').collect();
                if parts.len() == 3 {
                    if let (Ok(year), Ok(month), Ok(day)) = (
                        parts[0].parse::<i32>(),
                        parts[1].parse::<u32>(),
                        parts[2].parse::<u32>(),
                    ) {
                        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                            self.current_date_ad = date;
                            self.error_message = None;
                        } else {
                            self.error_message = Some("Invalid AD date".to_string());
                        }
                    } else {
                        self.error_message = Some("Invalid date format. Use YYYY-MM-DD".to_string());
                    }
                } else {
                    self.error_message = Some("Invalid date format. Use YYYY-MM-DD".to_string());
                }
            }
            InputMode::EnteringBS => {
                let parts: Vec<&str> = input.split(|c| c == '-' || c == '/').collect();
                if parts.len() == 3 {
                    if let (Ok(year), Ok(month), Ok(day)) = (
                        parts[0].parse::<u16>(),
                        parts[1].parse::<u8>(),
                        parts[2].parse::<u8>(),
                    ) {
                        match NepaliDate::new(year, month, day) {
                            Ok(bs_date) => {
                                match bs_to_ad(bs_date) {
                                    Ok(ad_date) => {
                                        self.current_date_ad = ad_date;
                                        self.error_message = None;
                                    }
                                    Err(e) => {
                                        self.error_message = Some(format!("Conversion error: {}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Invalid BS date: {}", e));
                            }
                        }
                    } else {
                        self.error_message = Some("Invalid date format. Use YYYY-MM-DD".to_string());
                    }
                } else {
                    self.error_message = Some("Invalid date format. Use YYYY-MM-DD".to_string());
                }
            }
            _ => {}
        }
        
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('h') | KeyCode::Left => app.move_day(-1),
                    KeyCode::Char('l') | KeyCode::Right => app.move_day(1),
                    KeyCode::Char('k') | KeyCode::Up => app.move_week(-1),
                    KeyCode::Char('j') | KeyCode::Down => app.move_week(1),
                    KeyCode::Char('H') | KeyCode::PageUp => app.move_month(-1),
                    KeyCode::Char('L') | KeyCode::PageDown => app.move_month(1),
                    KeyCode::Char('t') | KeyCode::Home => app.jump_to_today(),
                    KeyCode::Char('a') => {
                        app.input_mode = InputMode::EnteringAD;
                        app.input_buffer.clear();
                        app.error_message = None;
                    }
                    KeyCode::Char('b') => {
                        app.input_mode = InputMode::EnteringBS;
                        app.input_buffer.clear();
                        app.error_message = None;
                    }
                    KeyCode::Char('?') => {
                        app.input_mode = InputMode::Help;
                    }
                    _ => {}
                },
                InputMode::EnteringAD | InputMode::EnteringBS => match key.code {
                    KeyCode::Enter => app.handle_input_submit(),
                    KeyCode::Char(c) => {
                        app.input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                        app.input_buffer.clear();
                    }
                    _ => {}
                },
                InputMode::Help => {
                    app.input_mode = InputMode::Normal;
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Miti - Calendar Viewer (AD ↔ BS)")
        .style(app.theme.title_style())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[1]);

    // Calendar section
    let calendar_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    // AD Calendar
    let ad_calendar = render_ad_calendar(app);
    f.render_widget(ad_calendar, calendar_chunks[0]);

    // BS Calendar
    let bs_calendar = render_bs_calendar(app);
    f.render_widget(bs_calendar, calendar_chunks[1]);

    // Date info panel
    let date_info = ui::widgets::render_date_info(
        app.current_date_ad,
        app.error_message.as_ref(),
        &app.theme,
    );
    f.render_widget(date_info, main_chunks[1]);

    // Keybindings
    let keybindings = ui::widgets::render_keybindings(&app.theme);
    f.render_widget(keybindings, chunks[2]);

    // Render modals
    match app.input_mode {
        InputMode::EnteringAD => {
            let (modal_area, modal) = ui::widgets::render_input_modal(
                "AD",
                &app.input_buffer,
                f.area(),
                &app.theme,
            );
            f.render_widget(modal, modal_area);
        }
        InputMode::EnteringBS => {
            let (modal_area, modal) = ui::widgets::render_input_modal(
                "BS",
                &app.input_buffer,
                f.area(),
                &app.theme,
            );
            f.render_widget(modal, modal_area);
        }
        InputMode::Help => {
            let help_area = ui::help::get_help_modal_area(f.area());
            let help = ui::help::render_help_screen(&app.theme);
            f.render_widget(help, help_area);
        }
        _ => {}
    }
}

fn render_ad_calendar(app: &App) -> Paragraph {
    let date = app.current_date_ad;
    let today = Local::now().date_naive();
    
    let first_of_month = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();
    let days_in_month = get_days_in_month_ad(date.year(), date.month());
    
    let mut lines = vec![];
    
    lines.push(Line::from(vec![
        Span::styled(
            format!("{} {}", month_name_ad(date.month()), date.year()),
            app.theme.header_style(),
        ),
    ]));
    lines.push(Line::from(""));
    
    lines.push(Line::from(vec![
        Span::styled("Su Mo Tu We Th Fr Sa", app.theme.muted_style()),
    ]));
    
    let first_weekday = first_of_month.weekday().num_days_from_sunday();
    let mut current_line = vec![];
    
    for _ in 0..first_weekday {
        current_line.push(Span::raw("   "));
    }
    
    for day in 1..=days_in_month {
        let current_day = NaiveDate::from_ymd_opt(date.year(), date.month(), day).unwrap();
        let is_selected = day == date.day();
        let is_today = current_day == today;
        
        let style = if is_today {
            app.theme.today_style()
        } else if is_selected {
            app.theme.selected_style()
        } else {
            Style::default()
        };
        
        current_line.push(Span::styled(format!("{:2} ", day), style));
        
        if (first_weekday + day) % 7 == 0 {
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }
    
    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("AD Calendar"))
        .wrap(Wrap { trim: false })
}

fn render_bs_calendar(app: &App) -> Paragraph {
    let bs_date = match ad_to_bs(app.current_date_ad) {
        Ok(date) => date,
        Err(_) => {
            return Paragraph::new("Error converting to BS")
                .block(Block::default().borders(Borders::ALL).title("BS Calendar"));
        }
    };
    
    let today = Local::now().date_naive();
    let today_bs = ad_to_bs(today).ok();
    
    let days_in_month = match get_days_in_month(bs_date.year, bs_date.month) {
        Some(days) => days,
        None => {
            return Paragraph::new("Year not in supported range")
                .block(Block::default().borders(Borders::ALL).title("BS Calendar"));
        }
    };
    
    let first_bs = match NepaliDate::new(bs_date.year, bs_date.month, 1) {
        Ok(date) => date,
        Err(_) => {
            return Paragraph::new("Error creating BS date")
                .block(Block::default().borders(Borders::ALL).title("BS Calendar"));
        }
    };
    
    let first_ad = match bs_to_ad(first_bs) {
        Ok(date) => date,
        Err(_) => {
            return Paragraph::new("Error converting to AD")
                .block(Block::default().borders(Borders::ALL).title("BS Calendar"));
        }
    };
    
    let mut lines = vec![];
    
    lines.push(Line::from(vec![
        Span::styled(
            format!("{} {}", month_name_bs(bs_date.month), bs_date.year),
            app.theme.header_style(),
        ),
    ]));
    lines.push(Line::from(""));
    
    lines.push(Line::from(vec![
        Span::styled("Su Mo Tu We Th Fr Sa", app.theme.muted_style()),
    ]));
    
    let first_weekday = first_ad.weekday().num_days_from_sunday();
    let mut current_line = vec![];
    
    for _ in 0..first_weekday {
        current_line.push(Span::raw("   "));
    }
    
    for day in 1..=days_in_month {
        let is_selected = day == bs_date.day;
        let is_today = if let Some(ref tbs) = today_bs {
            tbs.year == bs_date.year && tbs.month == bs_date.month && tbs.day == day
        } else {
            false
        };
        
        let style = if is_today {
            app.theme.today_style()
        } else if is_selected {
            app.theme.selected_style()
        } else {
            Style::default()
        };
        
        current_line.push(Span::styled(format!("{:2} ", day), style));
        
        if (first_weekday as u8 + day) % 7 == 0 {
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
        }
    }
    
    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }
    
    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("BS Calendar"))
        .wrap(Wrap { trim: false })
}

fn month_name_ad(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

fn month_name_bs(month: u8) -> &'static str {
    match month {
        1 => "Baisakh",
        2 => "Jestha",
        3 => "Ashadh",
        4 => "Shrawan",
        5 => "Bhadra",
        6 => "Ashwin",
        7 => "Kartik",
        8 => "Mangsir",
        9 => "Poush",
        10 => "Magh",
        11 => "Falgun",
        12 => "Chaitra",
        _ => "Unknown",
    }
}

fn get_days_in_month_ad(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days() as u32
}
