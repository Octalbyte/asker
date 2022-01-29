pub struct Asker{
	fields: Vec<(String, bool, bool, String, Box<dyn Fn(&str) -> bool>)>	
}

fn truish(t: &str) -> bool{
	return true;
}

impl Asker {
    pub fn new(&self) -> Asker {
		return Asker{
			fields: vec![]
		};	
	}

	pub fn add(&mut self, name: &str, opts: Vec<&str>, check: Option<Box<dyn Fn(&str) -> bool>>) {
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
			helpcheck = i 
		}
	}
		
}

	pub fn boot(){

	}
}