mod lib;

fn main(){
    let mut fields: Vec<(&str, Vec<&str>, Option<lib::ask::Regex>)> = vec![];
    fields.push(
        ("email", vec![], None)
    );
    lib::ask::ask(fields);
}