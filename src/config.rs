/* dsc config protocol */

// struct that contains variable's name and its value

//! # DSC strings (DSC config)
//!
//! `dsc_lib` is a crate that contains a struct and functions
//! for parsing dsc strings or configs (files.dsc)

//! # Syntax of dsc strings and configs
//!
//! 1. Usage of variables and values
//! `foo: bar`
//!
//! it's literally `let foo = "bar"` in Rust!
//! Also you can use `""` if you want add many words in `foo`:
//! `foo: "bar bar bar"`
//!
//!
//! 2. Usage of comments
//! `# this line will be igmored by the parser!`
//!
//! Also you can use comments in lines with variables:
//! `foo: bar # foo = "bar"`
//! `foo2: "2xBar" # still comment`
//!
//!
//! but also you can put `#` in value for some variables:
//! `foo: "#" # foo = "#"`
//!
//!
//! 3. More about quotes in syntax:
//! You can use `''` instead of `""` and put them in them:
//! `foo1: "'_'"`
//! `foo2: 'foo: "#"'`

#[derive(Debug, PartialEq)]
pub struct Var {
    name:  String,
    value: String,
}

impl Var {
    /// returns a new `Var` instance
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let _ = Var::new("var", "val");
    /// ```
    pub fn new<'a>(name: &'a str, value: &'a str) -> Self {
        Self {
            name:  name.to_string(),
            value: value.to_string(),
        }
    }

    /// returns a reference to `Var.name`
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let var = Var::new("var", "val");
    /// let var_name = var.get_name();
    /// assert_eq!(var_name, "var");
    /// ```
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// returns a reference to `Var.value`
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let var = Var::new("var", "val");
    /// let var_value = var.get_value();
    /// assert_eq!(var_value, "val");
    /// ```
    pub fn get_value(&self) -> &String {
        &self.value
    }

    /// returns `true` if `Var.value` is not an empty string
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let var1 = Var::new("var", "val");
    /// assert!(var1.is_empty());
    ///
    /// let var2 = Var::new("var", "");
    /// #[should_panic]
    /// assert!(var2.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        *self.get_name() == "".to_string()
    }

    /// returns a `Var` instance from a string
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let var1 = Var::parse_line("var: val # comment");
    /// let var2 = Var::new("var", "val");
    /// assert_eq!(var1, var2);
    /// ```
    pub fn parse_line(line: &str) -> Self {
        let (mut name, mut value) = (String::new(), String::new());

        let mut iter = line.chars();

        // getting variable's name
        while let Some(c) = iter.next() {
            match c {
                '"' | '\'' => continue, // don't use that characters in names please
                ' ' | '\t' => continue,
                ':' => break,
                '#' => return Self { name, value },
                _   => name.push(c)
            }
        }

        // getting variable's value
        let (mut count, mut test_c) = (0u8, '_');
        while let Some(c) = iter.next() {
            match c {
                ' '  => if test_c == '\'' || test_c == '\"' && count != 2 { value.push(c); } else { continue; },
                '#'  => if test_c == '\'' || test_c == '\"' && count != 2 { value.push(c); } else { break; },
                '\"' => if test_c != '\'' { test_c = '\"'; count += 1; } else { value.push(c); },
                '\'' => if test_c != '\"' { test_c = '\''; count += 1; } else { value.push(c); },
                _    => value.push(c),
            }
        }

        Self {
            name,
            value,
        }
    }

    /// returns a vector that contains some `Var` instances using the parser
    /// (gets `Var` instances from lines only! it's useful when you need to parse a file)
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let file = std::fs::read_to_string("test.dsc").unwrap();
    /// let _ = Var::collect_to_vec(&file);
    /// ```
    pub fn collect_to_vec(st: &str) -> Vec<Self> {
        let mut v: Vec<Self> = Vec::new();

        for i in st.lines() {
            v.push(Self::parse_line(i));
        }

        v
    }

    /// writes a `Var` instance to the file
    /// returns `Result<(), String>`
    /// it is a method, not a function!
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let var = Var::new("var", "val");
    /// var.write_var("foo1.dsc");
    ///
    /// let st = std::fs::read_to_string("foo1.dsc").unwrap();
    /// assert_eq!("var: val", st);
    /// ```
    pub fn write_var(&self, filename: &str) -> Result<(), String> {
        use std::{
            fs::File,
            io::Write,
        };

        let st = format!("{}: {}", &self.name, &self.value);

        let mut file = match File::create(filename) {
            Ok(file) => file,
            Err(_) => return Err(String::from("Error: can't create the file!")),
        };

        match file.write_all(st.as_bytes()) {
            Ok(()) => Ok(()),
            Err(_) => Err(String::from("Error: can't write to the file!")),
        }
    }

    /// writes a vector of `Var` to the file,
    /// returns `Result<(), String>
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let st = " foo1: bar1
    /// foo2: bar2
    /// foo3: bar3
    /// ";
    ///
    /// let v = Var::collect_to_vec(st);
    /// let _ = Var::write_var_vec(&v, "foo2.dsc").unwrap();
    /// ```
    pub fn write_var_vec(v: &Vec<Self>, filename: &str) -> Result<(), String> {
        use std::{
            fs::File,
            io::Write,
        };

        let mut buffer = String::new();

        for i in v.iter() {
            let st = format!("{}: {}\n", i.get_name(), i.get_value());
            buffer.push_str(&st);
        }

        let mut file = match File::create(filename) {
            Ok(file) => file,
            Err(_) => return Err(String::from("Error: can't create the file!")),
        };

        match file.write_all(buffer.as_bytes()) {
            Ok(()) => Ok(()),
            Err(_) => Err(String::from("Error: can't write to the file!")),
        }
    }

    /// parses the file and returns `Resulr<Vec<Var>>, String>
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let vec = Var::parse_file("test.dsc");
    /// for i in vec.iter() {
    ///     println!("{:?}", i);
    /// }
    /// ```
    pub fn parse_file(file: &str) -> Result<Vec<Self>, String> {
         match std::fs::read_to_string(file) {
            Ok(st) => Ok(Self::collect_to_vec(&st)),
            Err(_) => Err(String::from("Error: can't read the file")),
        }
    }

    /// returns string that made from `Var` instance
    ///
    /// # Example
    /// ```
    /// use dsc_lib::*;
    ///
    /// let var = Var::new("var", "val");
    /// let st = Var::var_to_string(&var);
    /// assert_eq!("var: val", st);
    /// ```
    pub fn var_to_string(var: &Self) -> String {
        format!("{}: {}", var.name, var.value)
    }
}
