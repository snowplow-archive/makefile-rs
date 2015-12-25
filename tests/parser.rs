/*
 * Copyright (c) 2015 Snowplow Analytics Ltd. All rights reserved.
 *
 * This program is licensed to you under the Apache License Version 2.0,
 * and you may not use this file except in compliance with the Apache License Version 2.0.
 * You may obtain a copy of the Apache License Version 2.0 at http://www.apache.org/licenses/LICENSE-2.0.
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the Apache License Version 2.0 is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the Apache License Version 2.0 for the specific language governing permissions and limitations there under.
 */

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
