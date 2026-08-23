/* dsc config protocol */

// struct that contains variable's name and its value
pub struct Var {
    pub name:  String,
    pub value: String,
}

impl Var {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn new<'a>(value: &'a str, name: &'a str) -> Self {
        Self {
            value: value.to_string(),
            name:  name.to_string(),
        }
    }

    fn parse_line(line: &str) -> Self {
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

    pub fn collect_to_vec(st: &str) -> Vec<Self> {
        let mut v: Vec<Self> = Vec::new();

        for i in st.lines() {
            v.push(Self::parse_line(i));
        }

        v
    }

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

    pub fn write_var_vec(v: Vec<Self>, filename: &str) -> Result<(), String> {
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
}
