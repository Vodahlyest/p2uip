use serde::{Serialize, Deserialize};

pub enum AppRole{
    Host,
    Client,
    None,
}

pub enum AppScreen{
    SetupRole,
    SetupName,
    SetupIp,
    Chat,
}

pub enum InputMode{
    Normal,
    Editing,
}

#[derive(Serialize, Deserialize)]
pub struct Message{
    pub sender: String,
    pub contents: String,
}

pub struct AppState{
    pub this_machine: String,
    pub input_mode: InputMode,
    pub input_text: String,
    pub messages: Vec<String>,
    pub current_screen: AppScreen,
    pub target_ip: String,
    pub role: AppRole,
    pub quit_flag: bool,
}

impl AppState{
    pub fn new(input: String) -> Self{
        Self{
            this_machine: String::new(),
            input_mode: InputMode::Normal,
            input_text: input,
            messages: Vec::new(),
            current_screen: AppScreen::SetupRole,
            target_ip: String::new(),
            role: AppRole::None,
            quit_flag: false,
        }
    }
}