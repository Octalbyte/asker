use regex::Regex;
use std::io;
use std::io::Write;
use std::collections::HashMap;

	pub fn ask(
		fields: Vec<(&str, Vec<&str>, Option<Regex>)>	
	) -> (HashMap<String, String>, HashMap<String, bool>){

		let str_matches: HashMap<String, String> = HashMap::new();
		let bool_matches: HashMap<String, bool> = HashMap::new();

		for i in fields.iter(){
			let (name, opts, checker) = i;
			let mut hidden = false;
			let mut confirm = false;
			let mut isbool = false;
			let mut default = None; 
			let mut req = None;
			let mut id = name.to_owned();
			//let mut stuff: Vec<&str> = vec![];

			for wrd in opts.iter(){
				match wrd {
					&"hidden" => {
						hidden = true;	
					},
					&"confirm" => {
						confirm = true;	
					},
					&"isbool" => {
						isbool = true;
					}
					a => { 
						let b: String = String::from(a.to_owned());
						if b.starts_with("default:"){
							
							default = Some(b.replacen("default:","",1).as_str());
						}
						if b.starts_with("req:"){
							req = Some(b.replacen("req:", "",1).as_str());
						}
						if b.starts_with("id:"){
							id = b.replacen("id:", "",1).as_str();
						}
					}		
				}
			}
			
			let match_any = Regex::new(r".*").unwrap();
			let _reg = match checker {
				Some(exp) => exp,
				None => {
					&match_any
				}
			};
			/*
			if isbool {
				safe_print(name);
				match default {
					Some("true") => {
						safe_print(" (Y/n): ");
					},
					Some("false") => {
						safe_print(" (y/N): ");
					}
					Some(_) => {
						panic!("Invalid default for bool: Accepted values: false, true");
					}
					None => {
						safe_print(" (y/n): ");
					}
				}

			}
			*/
			/*
			print!("Enter a number: ");
			    io::stdout().flush().unwrap();
    			let mut val = String::new();

		    io::stdin().read_line(&mut val)
        		.expect("Error getting guess");
			
			*/
			
		}
		return (str_matches, bool_matches);
	}

	fn safe_print(a1: &str){
		print!("{}", a1);
		io::stdout().flush().unwrap();
	}