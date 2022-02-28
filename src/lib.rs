
#[cfg(test)]
pub mod tests {
    use regex::Regex;
    use crate::ask;
    #[test]
    fn test(){
        let fields: Vec<(&str, Vec<&str>, Option<ask::Regex>)> = vec![
            ("email", vec![""], Some(Regex::new(r".*@gmail.com").unwrap())),
            ("username", vec!["req: Must be only lower-case letters", "default:uglyoctopus"], Some(Regex::new(r"\p{Ll}").unwrap())),
            ("Are you logged?", vec!["isbool", "id:islogged"], None),
            ("password", vec!["hidden", "confirm"], None),
            ("In which year were you born", vec!["id:birth","default:1999"], Some(Regex::new(r"\p{Nd}").unwrap())),
            ("Do you agree to the Terms of Service?", vec!["id:terms", "isbool", "default:true"], None),
        ];
        let (mut f, mut b) = ask::ask(fields);

        println!("\nString results");

        for (index, value) in f.iter_mut() {
            println!("{} --> {}", index, value);
        }

        println!("\nBool results");

        for (index, value) in b.iter_mut() {
            println!("{} --> {}", index, value);
        }

        println!("");
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
            count += 1
        }

        let match_any = Regex::new(r".*").unwrap();
        let _reg = match checker {
            Some(exp) => exp,
            None => &match_any,
        };

        if isbool {
            

            loop {
                safe_print(name);
                let mut default_as_bool = None;
                match default {
                Some("default:true") => {
                    safe_print(" (Y/n): ");
                    default_as_bool = Some(true);
                }
                Some("default:false") => {
                    safe_print(" (y/N): ");
                    default_as_bool = Some(false);
                }
                Some(_) => {
                    panic!("Invalid default for bool: Accepted values: false, true");
                }
                None => {
                    safe_print(" (y/n): ");
                }
                }

                let id = id.replacen("id:","", 1);
                let id = id.as_str();


                match read().unwrap() {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Char('y') => {
                                safe_print("y\n");
                                bool_matches.insert(String::from(id), true);
                                break;
                            }
                            KeyCode::Char('Y') => {
                                safe_print("Y\n");
                                bool_matches.insert(String::from(id), true);
                                break;
                            }
                            KeyCode::Char('n') => {
                                safe_print("n\n");
                                bool_matches.insert(String::from(id), false);
                                break;
                            }
                            KeyCode::Char('N') => {
                                safe_print("N\n");
                                bool_matches.insert(String::from(id), false);
                                break;
                            }
                            KeyCode::Enter => {
                                safe_print("\n");
                                match default_as_bool {
                                    Some(td) => {
                                        bool_matches.insert(String::from(id), td);
                                        break;
                                    }
                                    None => {
                                        println!("You must input Y or N");
                                    }
                                }
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
                let mut hasdefault = false;
                match default {
                    Some(i) => {
                        let to_print = i.replacen("default:", "", 1);
                        safe_print(format!(" ({}):", to_print).as_str());
                        hasdefault = true;
                    }
                    None => {
                        safe_print(": ");
                    }
                }
                let mut line = getin::get_in(&hidden);
                if _reg.is_match(line.as_str()) || (line.as_str() == "" && hasdefault ){
                    if confirm {
                        safe_print(format!("Confirm {}", name).as_str());
                        match default {
                            Some(i) => {
                                let to_print = i.replacen("default:", "", 1);
                                safe_print(format!(" ({}):", to_print).as_str());
                            }
                            None => {
                                safe_print(": ");
                            }
                        }
                        let confline = getin::get_in(&hidden);
                        if line != confline {
                            println!("Fields do not match");
                            continue;
                        }
                    }
                    if line.as_str() == "" {
                        match default {
                            Some(td) => {
                                line = String::from(td.replacen("default:","",1));
                            },
                            _ => {}
                        }
                    }
                    str_matches.insert(String::from(id.replacen("id:","",1)), line);
                    break;
                } else {
                    match req {
                        None => {
                            println!("Field must match {}", (&_reg).to_owned());
                        }
                        Some(rq) => {
                            println!("Field requirements: {}", rq.replacen("req:", "", 1));
                        }
                    }
                }
            }
        }
    }
    (str_matches, bool_matches)
}


fn safe_print(a1: &str) {
    print!("{}", a1);
    io::stdout().flush().unwrap();
}


}

