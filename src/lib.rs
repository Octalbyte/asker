pub struct Asker<'a>{
	fields: Vec<(&'a str, bool, bool, &'a str, Box<dyn Fn(&str) -> bool>)>	
}

fn truish(t: &str) -> bool{
	return true;
}

impl Asker<'_> {
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
			helpcheck = i ;
		}
	}
	let mut checker = Box::new((&truish).to_owned());
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