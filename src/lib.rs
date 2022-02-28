
#[cfg(test)]
pub mod tests {
    use crate::ask;
    #[test]
    fn test(){
        let mut fields: Vec<(&str, Vec<&str>, Option<ask::Regex>)> = vec![];
        fields.push(
            ("email", vec![], None)
        );
        let (f, b) = ask::ask(fields);
        println!("{}", f);
    }
}


pub mod ask{
    pub use regex::Regex;
    use std::io;
use std::io::Write;
use std::collections::HashMap;
use crossterm;

use crossterm::event::Event;
use crossterm::event::read;
use crossterm::event::KeyCode;
mod getin;

pub fn ask(
    fields: Vec<(&str, Vec<&str>, Option<Regex>)>,
) -> (HashMap<String, String>, HashMap<String, bool>) {

    let mut str_matches: HashMap<String, String> = HashMap::new();
    let mut bool_matches: HashMap<String, bool> = HashMap::new();

    for i in fields.iter() {
        let (name, opts, checker) = i;
        let mut hidden = false;
        let mut confirm = false;
        let mut isbool = false;
        let mut default = None;
        let mut req = None;
        let mut id = name.to_owned();
        let mut count = 0;

        for wrd in opts.iter() {
            match wrd {
                &"hidden" => {
                    hidden = true;
                }
                &"confirm" => {
                    confirm = true;
                }
                &"isbool" => {
                    isbool = true;
                }
                _ => {
                    let b: String = String::from((&opts[count]).to_owned());
                    if b.starts_with("default:") {
                        default = Some((&opts[count]).to_owned());
                    }
                    if b.starts_with("req:") {
                        req = Some((&opts[count]).to_owned());
                    }
                    if b.starts_with("id:") {
                        id = (&opts[count]).to_owned();
                    }
                }	
            }
            count = count + 1
        }

        let match_any = Regex::new(r".*").unwrap();
        let _reg = match checker {
            Some(exp) => exp,
            None => &match_any,
        };

        if isbool {
            safe_print(name);
            match default {
                Some("default:true") => {
                    safe_print(" (Y/n): ");
                }
                Some("default:false") => {
                    safe_print(" (y/N): ");
                }
                Some(_) => {
                    panic!("Invalid default for bool: Accepted values: false, true");
                }
                None => {
                    safe_print(" (y/n): ");
                }
            }

            loop {
                match read().unwrap() {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Char('y') => {
                                safe_print("y");
                                bool_matches.insert(String::from(id), true);
                            }
                            KeyCode::Char('Y') => {
                                safe_print("Y");
                                bool_matches.insert(String::from(id), true);
                            }
                            KeyCode::Char('n') => {
                                safe_print("n");
                                bool_matches.insert(String::from(id), false);
                            }
                            KeyCode::Char('N') => {
                                safe_print("N");
                                bool_matches.insert(String::from(id), false);
                            }
                            KeyCode::Enter => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

            }



        } else {
            loop {
                safe_print(name);
                match default {
                    Some(i) => {
                        let to_print = i.replacen("default:", "", 1);
                        safe_print(format!(" ({}):", to_print).as_str());
                        break;
                    }
                    None => {
                        safe_print(": ");
                    }
                }
                let line = getin::get_in(&hidden);
                if _reg.is_match(&line.as_str()) {
                    if confirm {
                        let confline = getin::get_in(&hidden);
                        if line != confline {
                            println!("Fields dont match");
                            continue;
                        }
                    }
                    str_matches.insert(String::from(id), line);
                    break;
                } else {
                    match req {
                        None => {
                            println!("Field must match {}", (&_reg).to_owned());
                        }
                        Some(rq) => {
                            println!("Field must {}", rq);
                        }
                    }
                }
            }
        }
    }
    return (str_matches, bool_matches);
}


fn safe_print(a1: &str) {
    print!("{}", a1);
    io::stdout().flush().unwrap();
}


}

