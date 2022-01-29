pub struct Asker<'a>{
	fields: Vec<(&'a str, bool, bool, &'a str, Box< FnOnce(&str) -> bool>)>	
}



impl Asker<'_> {
    pub fn new(&self) -> Asker {
		return Asker{
			fields: vec![]
		};	
	}

	pub fn add(&mut self, name: &str, opts: Vec<&str>, check: Option<Box<FnOnce(&str) -> bool>) {
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
	let mut checker = Box::new(|| s: &str{
		return true;
	});
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