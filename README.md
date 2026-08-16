Now i wrote the dsc config as a little library that contains basic functions for proccessing configs

Example for .dsc you can find in test.dsc file of this repo

Usage exxample:
```rust
use dsc::config::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut dsc_collection: Vec<Var> = Vec::new();

    if let Ok(lines) = string_from_file(&args[1]) {
        dsc_collection = Var::collect_to_vec(&lines);
    }

    println!("=== Result ===");

    for i in dsc_collection.iter() {
        if i.get_name() != "".to_string() && i.get_value() != "".to_string() {
            println!("{}: {}", i.get_name(), i.get_value());
        }
    }
}


// my sugar function for more comfortable error handling
fn string_from_file(file: &str) -> Result<String, String> {
    match std::fs::read_to_string(file) {
        Ok(s) => Ok(s),
        Err(_) => Err(String::from("Error: can't read the file")),
    }
}
```
