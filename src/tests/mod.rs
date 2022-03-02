pub mod tests {
    use regex::Regex;
    use crate::ask;
    use crate::ask::FieldSet;

    #[test]
    fn test(){
        let fields: FieldSet = vec![
            ("email", vec![""], Some(Regex::new(r".*@gmail.com").unwrap())),
            ("username", vec!["req: Must be only lower-case letters", "default:uglyoctopus"], Some(Regex::new(r"^\p{Ll}+$").unwrap())),
            ("Are you logged?", vec!["isbool", "id:islogged"], None),
            ("password", vec!["hidden", "confirm"], None),
            ("In which year were you born", vec!["id:birth","default:1999"], Some(Regex::new(r"^\p{Nd}+$").unwrap())),
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