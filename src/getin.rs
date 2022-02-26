use crossterm;
use crate::safe_print;
use crossterm::event::Event;
use crossterm::event::read;
use crossterm::event::KeyCode;

pub fn get_in(hidden: &bool) -> String{
    let mut line = String::new();
    loop {
        match read().unwrap() {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char(ch) => {
                        let ch = format!("{}", ch);
                        if !hidden {
                            safe_print(&ch.as_str());
                        } else {
                            safe_print("*");
                        }

                        line = line + ch.as_str();
                    }
                    KeyCode::Backspace => {
                        safe_print("\u{8}");
                        line = line[..line.len() - 1].to_string();
                    }
                    KeyCode::Enter => {
                        safe_print("\n");
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

    }
    return line;
}