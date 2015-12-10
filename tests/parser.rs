
extern crate makefile;

#[macro_use]
extern crate nom;

use std::str::from_utf8;
use std::collections::HashMap;

use nom::{IResult};

use makefile::parser;

#[test]
fn parse_variables_test() {

    let ini_file = &b"foo=bar
foo_w_underscore=bar2
todo_another_test=bar3
"[..];

    let ini_after_parser = &b""[..];

    let res = parser::keys_and_values(ini_file);
    // println!("{:?}", res);
    match res {
        IResult::Done(i, ref o) => println!("i: {:?} | o: {:?}", from_utf8(i), o),
        _ => println!("error")
    }

    let mut expected_1: HashMap<&str, &str> = HashMap::new();
    expected_1.insert("foo", "bar");
    expected_1.insert("foo_w_underscore", "bar2");
    expected_1.insert("todo_another_test", "bar3");
    assert_eq!(res, IResult::Done(ini_after_parser, expected_1));
}
