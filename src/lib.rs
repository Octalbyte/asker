use regex::Regex;
use std::io;
use std::io::Write;

	pub fn ask(
		fields: Vec<(&str, Vec<&str>, Option<Regex>)>	
	) {

		for i in fields.iter(){
			let (name, opts, checker) = i;
			let mut hidden = false;
			let mut confirm = false;
			let mut req = "";
			
			for wrd in opts.iter(){
				match wrd {
					"hidden" => {
						hidden = true;	
					},
					"confirm" => {
						confirm = true;	
					},
					a => {
						req = a;
					}
						
				}
			}
			
			
			let reg = match checker {
				Some(exp) => exp,
				None => Regex::new(r".*")
			}
			/*
			print!("Enter a number: ");
			    io::stdout().flush().unwrap();
    			let mut val = String::new();

		    io::stdin().read_line(&mut val)
        		.expect("Error getting guess");
			
			*/
			
		}

	}
