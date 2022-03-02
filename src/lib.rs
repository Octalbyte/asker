
#[cfg(test)]
pub mod tests;


pub mod ask{
    pub use regex::Regex;
    use std::collections::HashMap;

    
    mod askbool;
    mod askstring;

    pub type FieldSet<'a> = Vec<(&'a str, Vec<&'a str>, Option<Regex>)>;

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

        let reg = match checker {
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
            askstring::askstring(name, default, hidden, confirm, reg, req, id, &mut str_matches);
        }
    }
    (str_matches, bool_matches)
}
}

