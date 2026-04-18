use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

mod drawing_and_catching;
use drawing_and_catching::screen_drawing::screen_drawing;
use drawing_and_catching::keyboard_catching::keyboard_catching;

use tokio::{sync::mpsc};
mod app;
use app::AppState;

use crate::app::Message;

//цикл работы приложения
async fn run_app(term: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut AppState, mut rx_net: Option<mpsc::Receiver<app::Message>>, mut tx_ui: Option<mpsc::Sender<Message>>, tx_net: mpsc::Sender<Message>, mut rx_ui: mpsc::Receiver<Message>) -> io::Result<()>{
    loop{
        //отрисовка экрана
        screen_drawing(term, app)?;

        keyboard_catching(app, &mut rx_net, &mut tx_ui, &tx_net, &mut rx_ui)?;

        if app.quit_flag == true{
            break Ok(());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    //инициализация mpsc: интерфейс
    let (tx_ui, rx_ui) = mpsc::channel::<app::Message>(50);
    
    //инициализация mpsc: сеть
    let (tx_net, rx_net) = mpsc::channel::<app::Message>(50);
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    //backend - транслирует команды ratatui в ANSI последовательности, которые
    //извлекает уже terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_state: AppState = AppState::new("erase this to type".to_string());

    run_app(&mut terminal, &mut current_state, Some(rx_net), Some(tx_ui), tx_net, rx_ui).await?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
