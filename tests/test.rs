use dsc::config::*;

#[test]
fn parsing_test1() {
    let var1 = Var::new("var", "val");
    let var2 = Var::parse_line("var: val #comment");
    assert_eq!(var1, var2);
}

#[test]
fn parsing_test2() {
    let var1 = Var::new("var", "this '#' is supported!");
    let var2 = Var::parse_line("var: \"this '#' is supported!\" # should works");
    assert_eq!(var1, var2);
}

#[test]
fn parsing_test3() {
    let var1 = Var::new("var", "");
    let var2 = Var::parse_line("var: # should works too");
    assert_eq!(var1, var2);
}

#[test]
fn get_test() {
    let var = Var::parse_line("var: val");
    let name = var.get_name();
    let value = var.get_value();
    assert_eq!("var", name);
    assert_eq!("val", value);
}

#[test]
fn file_parsing() {
    let vec = Var::collect_to_vec(&std::fs::read_to_string("test.dsc").unwrap());

    if let Ok(v) = Var::parse_file("test.dsc") {
        assert_eq!(v, vec);
    }
}
