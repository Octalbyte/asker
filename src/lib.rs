use regex::Regex;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use crossterm::{
	self, Event, read
}

	pub fn ask(
		fields: Vec<(&str, Vec<&str>, Option<Regex>)>	
	) -> (HashMap<String, String>, HashMap<String, bool>){

		let mut str_matches: HashMap<String, String> = HashMap::new();
		let mut bool_matches: HashMap<String, bool> = HashMap::new();

		for i in fields.iter(){
			let (name, opts, checker) = i;
			let mut hidden = false;
			let mut confirm = false;
			let mut isbool = false;
			let mut default = None; 
			let mut req = None;
			let mut id = name.to_owned();
			let mut count = 0;

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
					_ => { 
						let b: String = String::from((&opts[count]).to_owned());
						if b.starts_with("default:"){
							default = Some((&opts[count]).to_owned());
						}
						if b.starts_with("req:"){
							req = Some((&opts[count]).to_owned());
						}
						if b.starts_with("id:"){
							id = (&opts[count]).to_owned();
						}
					}		
				}
				count = count+1
			}
			
			let match_any = Regex::new(r".*").unwrap();
			let _reg = match checker {
				Some(exp) => exp,
				None => {
					&match_any
				}
			};
			
			if isbool {
				safe_print(name);
				match default {
					Some("default:true") => {
						safe_print(" (Y/n): ");
					},
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
					// `read()` blocks until an `Event` is available
					match read().unwrap() {
						Event::Key(event) => {
							match event.code {
								Keycode::Char('y') => {
									safe_print("y");
									bool_matches.insert(String::from(id), true);
								}
								Keycode::Char('Y') => {
									safe_print("Y");
									bool_matches.insert(String::from(id), true);
								}
								Keycode::Char('n') => {
									safe_print("n");
									bool_matches.insert(String::from(id), false);
								}
								Keycode::Char('N') => {
									safe_print("N");
									bool_matches.insert(String::from(id), false);
								}
								Keycode::Enter => {

								}
							}
						},
						_ => {}
					}
				}

			}
			

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