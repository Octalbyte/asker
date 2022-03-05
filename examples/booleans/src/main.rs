use asker::ask;

fn main() {
    let mut fields: ask::FieldSet = vec![];
    fields.push(("terms", vec![], None));
    ask::ask(fields);
    println!()
}