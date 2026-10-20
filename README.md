The dsc config and lines parser as a little crate that contains basic functions for proccessing configs

Usage example:
```rust
use dsc_lib::*;

fn main() {
    if let Ok(v) = Var::parse_file("test.dsc") {
        println!("=== Result ===");

        for i in v.iter() {
            if *i.get_name() != "".to_string() && !i.is_empty() {
                println!("{}: {}", i.get_name(), i.get_value());
            }
        }
    }
}
```
