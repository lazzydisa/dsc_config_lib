/* dsc config protocol */

// struct that contains variable's name and its value
pub struct Var {
    pub name:  String,
    pub value: String,
}

impl Var {
    pub fn get_name(&self) -> String {
        self.name.clone()   // anyway you need to copy of the original string
                            // that's why you will create something like this method
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn parse_line(line: &str) -> Self {
        let (mut name, mut value) = (String::new(), String::new());

        // getting variable's name
        for var in line.chars() {
            match var {
                '"' | '\'' => continue, // don't use that characters in names please
                ' ' | '\t' => continue,
                '#' | ':'  => break,
                _ => name.push(var)
            }
        }

        // getting variable's value
        let (mut test1, mut test2, mut c_test) = (false, false, 'x');
        for val in line.chars() {
            if val != ':' && test1 == false { continue; }

            test1 = true;

            match val {
                ':'  => continue,
                '"'  => if c_test != '\'' {
                    if test2 == false {
                        test2 = true;
                        c_test = '"';
                        continue;
                    } else {
                        break;
                    }
                } else { value.push(val); },
                '\'' => if c_test != '"' {
                    if test2 == false { 
                        test2 = true;
                        c_test = '\'';
                        continue;
                    } else {
                        break;
                    }
                } else { value.push(val); },
                ' '  => if test2 == false { continue; } else { value.push(val); },
                '#'  => if test2 == false { break; } else { value.push(val); },
                _    => value.push(val),
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
}
