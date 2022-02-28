

mod lib;

!#[cfg(test)]
fn test(){
    let mut fields: Vec<(&str, Vec<&str>, Option<lib::ask::Regex>)> = vec![];
    fields.push(
        ("email", vec![], None)
    );
    lib::ask::ask(fields);
}