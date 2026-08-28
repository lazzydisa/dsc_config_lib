Now i wrote the dsc config as a little library that contains basic functions for proccessing configs

Example for .dsc you can find in test.dsc file of this repo

Usage exxample:
```rust
use dsc::config::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //let mut dsc_collection: Vec<Var> = Vec::new();

    if let Ok(v) = Var::parse_file(&args[1]) {
        println!("=== Result ===");

        for i in v.iter() {
            if *i.get_name() != "".to_string() && *i.get_value() != "".to_string() {
                println!("{}: {}", i.get_name(), i.get_value());
            }
        }
    }
}
```
