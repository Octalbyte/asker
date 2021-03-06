use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use safe_print::safe_print;
use std::collections::HashMap;

pub fn ask_bool(
    name: &str,
    default: Option<&str>,
    id: &str,
    bool_matches: &mut HashMap<String, bool>,
) {
    loop {
        safe_print(" \x1b[32m ? \x1b[0m");
        safe_print(name);
        let mut default_as_bool = None;
        let mut default_as_str = None;
        match default {
            Some("default:true") => {
                safe_print(" (Y/n): ");
                default_as_bool = Some(true);
                default_as_str = Some("true");
            }
            Some("default:false") => {
                safe_print(" (y/N): ");
                default_as_bool = Some(false);
                default_as_str = Some("false");
            }
            Some(_) => {
                panic!("Invalid default for bool: Accepted values: false, true");
            }
            None => {
                safe_print(" (y/n): ");
            }
        }

        let id = id.replacen("id:", "", 1);
        let id = id.as_str();

        match read().unwrap() {
            Event::Key(event) => match event.code {
                KeyCode::Char('y') => {
                    safe_print("\x1b[36my\x1b[0m\n");
                    bool_matches.insert(String::from(id), true);
                    break;
                }
                KeyCode::Char('Y') => {
                    safe_print("\x1b[36mY\x1b[0m\n");
                    bool_matches.insert(String::from(id), true);
                    break;
                }
                KeyCode::Char('n') => {
                    safe_print("\x1b[36mn\x1b[0m\n");
                    bool_matches.insert(String::from(id), false);
                    break;
                }
                KeyCode::Char('N') => {
                    safe_print("\x1b[36mN\x1b[0m\n");
                    bool_matches.insert(String::from(id), false);
                    break;
                }
                KeyCode::Enter => {
                    
                    match default_as_bool {
                        Some(td) => {
                            match default_as_str {
                                Some(st) => {
                                  if st == "true" {
                                  safe_print(&("\x1b[36mY\x1b[0m"));
                                  } else {
                                    safe_print(&("\x1b[36mN\x1b[0m"));
                                  }

                                  safe_print("\n");
                                },
                                _ => {
                                    safe_print("\n");
                                }
                            }
                            bool_matches.insert(String::from(id), td);
                            break;
                        }
                        None => {
                            println!("You must input Y or N");
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
