pub struct Asker{
	fields: Vec<(String, bool, bool, String, dyn Fn(&str) -> bool)>	
}

fn truish(t: &str){
	return true;
}

impl Asker {
    pub fn new(&self) -> Asker {
		return Asker{
			fields: vec![]
		};	
	}

	pub fn add(&mut self, ) {
		self.fields.push();
	}

	pub fn boot(){

	}
}