use safe_print::safe_print;
use std::collections::HashMap;

mod getin;

pub fn askstring(
    name: &&str,
    default: Option<&str>,
    hidden: bool,
    confirm: bool,
    _reg: regex::Regex,
    req: Option<&str>,
    id: &str,
    str_matches: &mut HashMap<String, String>,
) {
    loop {
        safe_print(" \x1b[32m ? \x1b[0m");
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
        if _reg.is_match(line.as_str()) || (line.as_str() == "" && hasdefault) {
            if confirm {
                safe_print(" \x1b[32m ? \x1b[0m");
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
                        line = String::from(td.replacen("default:", "", 1));
                    }
                    _ => {}
                }
            }
            str_matches.insert(String::from(id.replacen("id:", "", 1)), line);
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
