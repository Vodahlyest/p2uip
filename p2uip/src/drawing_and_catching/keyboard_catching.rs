use tokio::{sync::mpsc, net::{TcpStream, TcpListener}};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::app::{InputMode::{Normal, Editing}, AppState, AppScreen::{SetupIp, SetupName, Chat, SetupRole}, AppRole};
use std::io;
use crate::app::Message;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
};
use local_ip_address::local_ip;

pub fn keyboard_catching(app: &mut AppState, rx_net: &mut Option<mpsc::Receiver<Message>>, tx_ui: &mut Option<mpsc::Sender<Message>>, tx_net: &mpsc::Sender<Message>, rx_ui: &mut mpsc::Receiver<Message>) -> io::Result<()>{
    if let Ok(incoming_message) = rx_ui.try_recv(){
        app.messages.push(format!("{}: {}", incoming_message.sender, incoming_message.contents));
    }

    if event::poll(std::time::Duration::from_millis(16))?{
        if let Event::Key(key) = event::read()?{
            if key.kind == KeyEventKind::Press{
                match app.current_screen{
                    SetupName => {
                        match key.code{
                            KeyCode::Char(i) => {app.input_text.push(i)},
                            KeyCode::Backspace => {
                                app.input_text.pop();
                            },
                            KeyCode::Esc => {
                                app.quit_flag = true;
                            }
                            KeyCode::Enter => {
                                match app.role{
                                    AppRole::Client => {
                                        app.this_machine = app.input_text.to_string();
                                        app.input_text.clear();
                                        app.current_screen = SetupIp;
                                    }
                                    AppRole::Host => {
                                        app.this_machine = app.input_text.to_string();
                                        app.current_screen = Chat;
                                        let network_ip = local_ip().unwrap();
                                        app.messages.push(format!("Сервер запущен. Подключение по адрессу {}:8080. Ждем подключения...", network_ip));
                                        app.input_text.clear();
                                        app.current_screen = Chat;
                                        let mut rx = rx_net.take().unwrap();
                                        let tx = tx_ui.take().unwrap();

                                        tokio::spawn(async move{
                                            match TcpListener::bind("0.0.0.0:8080").await {
                                                Ok(listener) => {
                                                if let Ok((socket, _addr)) = listener.accept().await{                                
                                                        let (mut reader, mut writer) = socket.into_split();
                                                        tokio::spawn(async move{
                                                            loop{
                                                                let mut read_buff: [u8; 1024] = [0; 1024];
                                                                let received_data: usize = reader.read(&mut read_buff).await.unwrap();
                                                                if received_data == 0{
                                                                    break;
                                                                }
                                                                let msg_string = String::from_utf8_lossy(&read_buff[..received_data]);
                                                                let message_json: Message = serde_json::from_str(&msg_string).unwrap();
                                                            tx.send(message_json).await.unwrap();
                                                            }
                                                        });
                                                            
                                                        tokio::spawn(async move{
                                                            loop{
                                                                let message = rx.recv().await.unwrap();
                                                                let message_string = serde_json::to_string(&message).unwrap();
                                                                writer.write_all(message_string.as_bytes()).await.unwrap();
                                                            }
                                                        });
                                                    }
                                                }
                                                Err(_) => ()
                                            }
                                        });
                                    }
                                    _ => ()
                                }
                               },
                            _ => ()
                        }
                    }
                    SetupIp => {
                        match key.code{
                            KeyCode::Char(i) => {app.input_text.push(i)},
                            KeyCode::Backspace => {
                                app.input_text.pop();
                            },
                            KeyCode::Esc => {
                                app.quit_flag = true;
                            }
                            KeyCode::Enter => {
                                app.target_ip = app.input_text.clone();
                                app.input_text.clear();
                                app.current_screen = Chat;
                                let mut rx = rx_net.take().unwrap();
                                let tx = tx_ui.take().unwrap();
                                let ip = app.target_ip.clone();
                                tokio::spawn(async move{
                                    match TcpStream::connect(ip).await{
                                        Ok(socket) => {
                                            let (mut reader, mut writer) = socket.into_split();
                                            tokio::spawn(async move{
                                                loop{
                                                    let mut read_buff: [u8; 1024] = [0; 1024];
                                                    let received_data: usize = reader.read(&mut read_buff).await.unwrap();
                                                    if received_data == 0{
                                                        break;
                                                    }
                                                    let msg_string = String::from_utf8_lossy(&read_buff[..received_data]);
                                                    let message_json: Message = serde_json::from_str(&msg_string).unwrap();
                                                    tx.send(message_json).await.unwrap();
                                                    }
                                                });
                                                
                                                tokio::spawn(async move{
                                                        loop{                                                            let message = rx.recv().await.unwrap();
                                                        let message_string = serde_json::to_string(&message).unwrap();
                                                        writer.write_all(message_string.as_bytes()).await.unwrap();
                                                    }
                                                });
                                        }
                                            Err(_) => ()
                                        }
                                    });
                                }
                            _ => ()
                        }
                    }
                    Chat => {
                        match (&mut app.input_mode, key.code) {
                            (Normal, KeyCode::Char('i')) => {app.input_mode = Editing},
                            (Normal, KeyCode::Esc) => {app.quit_flag = true;},
                            (Editing, KeyCode::Esc) => {app.input_mode = Normal},
                            (Editing, KeyCode::Char(c)) => {app.input_text.push(c)},
                            (Editing, KeyCode::Backspace) => {
                                //при нажатии backspace - из сообщения удаляется символ
                                //сообщение - массив char, что собственно и есть String
                                app.input_text.pop();
                            },
                            (Editing, KeyCode::Enter) => {
                                //при отправке сообщения - формируем его в структуру
                                //отправляя структуру, также добавляем введенное сообщение в историю диалога
                                let sender_name = &app.this_machine;
                                let message = Message{
                                    sender: sender_name.to_string(),
                                    contents: app.input_text.to_string(),
                                };

                                let formatted_message = format!("{}: {}", &message.sender, &message.contents);

                                if let Ok(()) = tx_net.try_send(message){
                                    app.messages.push(formatted_message);
                                    app.input_text.clear();
                                };
                            },
                            _ => {},
                        }
                    }
                    SetupRole => {
                        match key.code {
                            KeyCode::Char('1') => {
                                app.role = AppRole::Client;
                                app.current_screen = SetupName;
                            }
                            KeyCode::Char('2') => {
                                app.role = AppRole::Host;
                                app.current_screen = SetupName;
                            }
                            KeyCode::Esc => {
                                app.quit_flag = true;
                            }
                            _ => ()
                        }
                    }
                }
            }
        }
    }
    Ok(())
}