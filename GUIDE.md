# Guide

A quick tutorial to have a grasp of all asker's features.

Asker's main function is `asker::ask::ask`. It takes as argument a value of 
type `asker::ask::FieldSet`, which is an alias for a `Vec` consisting of 
the tuple type 
```rust 
(name: &str, option: Vec<&str>, checker: Option<asker::ask::Regex>)
```
 