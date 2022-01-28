pub struct Field{
	text: &'static str,
	confirm: bool,
	hidden: bool,
	condition: Option<&'static str>,
	checker: Option<dyn FnOnce(&str) -> bool>
}

pub struct Asker{
	fields: Vec<Field>	
}
pub fn defaults(){
	return Field{
		text: "Example field",
		confirm: false,
		hidden: false,
		condition: None,
		checker: None
	};
}

impl Asker {
    pub fn new(&self) -> Asker {
		return Asker{
			fields: vec![]
		};	
	}

	pub fn add(&mut self, f: Field) {
		self.fields.push(f);
	}

	pub fn boot(){

	}
}