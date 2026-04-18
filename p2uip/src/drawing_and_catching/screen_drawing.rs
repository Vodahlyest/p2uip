use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, BorderType},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use crate::app::{InputMode::{Normal, Editing}, AppState, AppScreen::{SetupIp, SetupName, Chat, SetupRole}};

pub fn screen_drawing(term: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut AppState) -> io::Result<()>{
    //отрисовка экрана
    term.draw(|f|{
        match app.current_screen{
            Chat => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(f.area());

                let input_field_border = match app.input_mode{
                    Normal => Block::new()
                        .border_style(
                            Style::new().bold().white()
                        )
                        .border_type(BorderType::HeavyDoubleDashed)
                        .borders(Borders::all())
                        .title("Нажми i для ввода сообщения"),
                    Editing => Block::new()
                        .border_style(
                            Style::new().bold().yellow()
                        )
                        .border_type(BorderType::HeavyDoubleDashed)
                        .borders(Borders::all())
                        .title("Режим ввода (Enter - отправка сообщений, Esc - Назад)"),
                        
                };

                let chat_list_border = Block::new()
                    .style(
                        Style::new()
                            .bold()
                            .white())
                    .title("Чат")
                    .border_type(BorderType::Rounded)
                    .borders(Borders::all());
                    
                let text_field = Paragraph::new(app.input_text.as_str()).block(input_field_border);

                let message_list: Vec<ListItem> = app.messages.iter().map(|m| ListItem::new(m.as_str())).collect::<Vec<_>>();
                let list_holder = List::new(message_list).block(chat_list_border);

                f.render_widget(list_holder, chunks[0]);
                f.render_widget(text_field, chunks[1]);
            }
            SetupName => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Length(3),
                        Constraint::Percentage(50),
                    ])
                    .split(f.area());

                let input_name_border = Block::new()
                        .border_style(Style::new().bold().white())
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded)
                        .title("Введите Имя");
                let name_input = Paragraph::new(app.input_text.as_str())
                        .block(input_name_border);

                f.render_widget(name_input, chunks[1]);
            }
            SetupIp => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Length(3),
                        Constraint::Percentage(50),
                    ])
                    .split(f.area());

                let input_ip_border = Block::new()
                        .border_style(Style::new().bold().white())
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded)
                        .title("Введите IP собеседника");
                let ip_input = Paragraph::new(app.input_text.as_str())
                        .block(input_ip_border);

                f.render_widget(ip_input, chunks[1]);
            }
            SetupRole => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Length(4),
                        Constraint::Percentage(50),
                    ])
                    .split(f.area());

                let settings_box = Block::new()
                        .border_style(Style::new().white())
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded)
                        .title("Выберите режим работы: ");
                let settings_note = Paragraph::new("1. Режим клиента\n2. Режим сервера")
                        .block(settings_box);
                    
                f.render_widget(settings_note, chunks[1]);
            }
        }
    })?;
    Ok(())
}