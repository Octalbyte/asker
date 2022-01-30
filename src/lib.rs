use regex::Regex;

pub struct Asker<'a>{
	fields: Vec<(&'a str, bool, bool, &'a str, &'a Regex)>	
}



impl Asker {
    pub fn new(&self) -> Asker {
		return Asker{
			fields: vec![]
		};	
	}

	pub fn add(&mut self, name: &str, opts: Vec<&str>, check: Option<&Regex>) {
	let mut hidden = false;
	if opts.iter().any(|&i| i=="hidden") {
    	hidden = true;
	}
	let mut confirm = false;
	if opts.iter().any(|&i| i=="hidden") {
    	confirm = true;
	}
	
	let mut helpcheck = "";
	for i in opts.iter() {
		if i != &"hidden" && i != &"confirm"{
			helpcheck = i ;
		}
	}
	
	let checker = &Regex::new(r".*").unwrap();
		
	match check {
		Some(i) => {
			checker = i;
		},
		None => {

		}
	
	}
	
	self.fields.push((name, hidden, confirm, helpcheck, checker));	
}

	pub fn boot(){

	}
}