    use asker::ask;
    use asker::ask::FieldSet;

    /* 
	Demonstrates how to use the id: property to have the user see a pretty
	question and at the same time have convenient HashMap entries
    */

    fn main(){
        let fields: FieldSet = vec![
            ("In which year were you born", vec!["id:birth","default:1999"], Some(Regex::new(r"^\p{Nd}+$").unwrap())),
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