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
extern crate daggy;

use std::collections::HashMap;

use daggy::Dag;

use makefile::types::Makefile;
use makefile::writer::write_to_string;

#[test]
fn write_makefile_test() {

    let mut vars = HashMap::new();
    vars.insert(String::from("foo"), String::from("bar"));
    vars.insert(String::from("foo_w_underscore"), String::from("bar2"));
    vars.insert(String::from("todo_another_test"), String::from("bar3"));

    let makefile = Makefile {
        variables: vars,
        rules: Vec::new(),
        dag: Dag::new()
    };

    let expected = String::from("todo_another_test='bar3'
foo='bar'
foo_w_underscore='bar2'
");

    let actual = write_to_string(makefile);
    assert_eq!(actual, expected);
}