# Guide

A quick tutorial to have a grasp of all asker's features.

## Intro 

Asker's main function is `asker::ask::ask`. It takes as argument a value of 
type `asker::ask::FieldSet`, which is an alias for a `Vec` consisting of 
the tuple type 
```rust 
(name: &str, options: Vec<&str>, checker: Option<asker::ask::Regex>)
```

The simplest example is:

```rust
asker::ask::ask(
	vec![
		("name", vec![], None)
	]
);

```

It asks for a field of value name, which will be displayed as `name:` to the user,
and is not verified nor has any special property.


There are three boolean options:

- `hidden`, which will hide what the user types and instead show asterisks,
- `confirm`, which will ask for the user to retype the field
- `isbool`, which will ask the field as boolean (Y or N)

There are also string options:

- `default:thedefault`, which will use the value `thedefault` as default
- `id:theid`, which sets the field entry in the HashMap (see return types below) to `theid`
- `req:thereq`, which will display the text `thereq` to the user if the first field and the confirm field do not match.


## Validating user input

The last value in the tuple is an `Option<asker::ask::Regex`. If it is `None`,
the string field will match anything. If it is `Some(regex)`, the field will
be validated with `regex`.

## Return types

`asker::ask::ask` returns `(HashMap<String, String>, HashMap<String, bool)`

The first value is the hashmap with the String values, and the other is
the hashmap with boolean values.

## Demonstration

Now let's take another look at the README example:

```rust

    use asker::ask::Regex;
    use asker::ask;
    use asker::ask::FieldSet;

    // This example demonstrates basic usage, and main features

    fn main(){
        let fields: FieldSet = vec![
            ("email", vec![""], Some(Regex::new(r".*@gmail.com").unwrap())), // A field with no properties, but a cheking regex
            ("username", vec!["req: Must be only lower-case letters", "default:uglyoctopus"], Some(Regex::new(r"^\p{Ll}+$").unwrap())), // A field that will display the requirements if validation fails
            ("Are you logged?", vec!["isbool", "id:islogged"], None), // A boolean field with no default
            ("password", vec!["hidden", "confirm"], None), // A hidden field that needs confirmation
            ("In which year were you born", vec!["id:birth","default:1999"], Some(Regex::new(r"^\p{Nd}+$").unwrap())), // A field with defaults, that will display its raw Regex if validation fails
            ("Do you agree to the Terms of Service?", vec!["id:terms", "isbool", "default:true"], None), // A boolean field with default
        ];
        let (mut f, mut b) = ask::ask(fields); // ask returns a tuple

        println!("\nString results");

        for (index, value) in f.iter_mut() { // Hashmaps are iterable
            println!("{} --> {}", index, value);
        }

        println!("\nBool results");

        for (index, value) in b.iter_mut() { // This one also is
            println!("{} --> {}", index, value);
        }

        println!("");
    }

```


Output: 
![image](https://user-images.githubusercontent.com/66487668/156898233-cecd394f-29d3-4cbb-97a7-6dcd7a44769b.png)


## Errors

```asker::ask::ask``` doesnt have any error types and will only `panic!`
if the default value for bool isn't `true` or `false` ([see the code](https://github.com/Octalbyte/asker/blob/main/src/ask/askbool/mod.rs#:~:text=mut%20default_as_bool%20%3D%20None%3B-,match%20default%20%7B,%7D,-%7D))

