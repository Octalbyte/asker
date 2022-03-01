# asker (WIP)	
Input prompt, for Rust (WIP)

FOR NOW THE CRATE NAME IS PARKED WHILE DEVELOPMENT OCCURS

visit [the github](https://github.com/Octalbyte/asker)

## Install
```toml
[dependencies]
asker = "0.1.0"
```

## Usage 

For a complete guide, check [GUIDE.md](https://github.com/Octalbyte/asker/blob/main/GUIDE.md) and [examples](https://github.com/Octalbyte/asker/tree/main/examples)

```rust
use asker::ask;
use asker::Regex;

// This example demonstrates the library's features

fn main(){
        let fields: Vec<(&str, Vec<&str>, Option<ask::Regex>)> = vec![
            ("email", vec![""], Some(Regex::new(r".*@gmail.com").unwrap())),
            ("username", vec!["req: Must be only lower-case letters", "default:uglyoctopus"], Some(Regex::new(r"\p{Ll}").unwrap())),
            ("Are you logged?", vec!["isbool", "id:islogged"], None),
            ("password", vec!["hidden", "confirm"], None),
            ("In which year were you born", vec!["id:birth","default:1999"], Some(Regex::new(r"\p{Nd}").unwrap())),
            ("Do you agree to the Terms of Service?", vec!["id:terms", "isbool", "default:true"], None),
        ];
        let (mut f, mut b) = ask::ask(fields);

        println!("\nString results");

        for (index, value) in f.iter_mut() {
            println!("{} --> {}", index, value);
        }

        println!("\nBool results");

        for (index, value) in b.iter_mut() {
            println!("{} --> {}", index, value);
        }

        println!("");
    }

```

Output:

![image](https://user-images.githubusercontent.com/66487668/156159807-1855ab60-b51c-40e1-a457-bfda74267611.png)
