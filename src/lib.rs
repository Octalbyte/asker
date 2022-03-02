
#[cfg(test)]
pub mod tests;


pub mod ask{
    pub use regex::Regex;
use std::collections::HashMap;


mod getin;
mod askbool;
use safe_print::safe_print;

type FieldSet<'a> = Vec<(&'a str, Vec<&'a str>, Option<Regex>)>;

pub fn ask(fields: FieldSet) -> (HashMap<String, String>, HashMap<String, bool>) {

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
                &"hidden" => {hidden = true;}
                &"confirm" => {confirm = true}
                &"isbool" => {isbool = true;}
                _ => {
                    let b: String = String::from((&opts[count]).to_owned());
                    if b.starts_with("default:") {default = Some((&opts[count]).to_owned());}
                    if b.starts_with("req:") {req = Some((&opts[count]).to_owned());}
                    if b.starts_with("id:") {id = (&opts[count]).to_owned();}
                }	
            }
            count += 1
        }

        let _reg = match checker {
            Some(exp) => exp.to_owned(),
            None => Regex::new(r".*").unwrap()
        };

        if isbool {
            
            askbool::ask_bool(
                name,
                default,
                id,
                &mut bool_matches
            )
                
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
}

