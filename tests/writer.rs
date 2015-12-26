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

use makefile::types::{Makefile, MakeRule};
use makefile::writer::write_to_string;

#[test]
fn write_makefile_test() {

    let vars = {
        let mut hm = HashMap::new();
        hm.insert(String::from("foo"), String::from("bar"));
        hm.insert(String::from("z"), String::from("bar2"));
        hm.insert(String::from("a"), String::from("bar3"));
        hm
    };

    let rules = {
        let a = MakeRule {
            target: String::from("done"),
            recipe: None,
            prerequisites: vec![String::from("sql-runner")]
        };
        let b = MakeRule {
            target: String::from("emr-etl-runner"),
            recipe: Some(String::from("./emr-etl-runner.sh foo bar")),
            prerequisites: vec![]
        };
        let c = MakeRule {
            target: String::from("attach-eips"),
            recipe: Some(String::from("/opt/attach-eips.py test")),
            prerequisites: vec![]
        };
        let d = MakeRule {
            target: String::from("storage-loader"),
            recipe: Some(String::from("./storage-loader.sh alpha beta")),
            prerequisites: vec![String::from("emr-etl-runner"), String::from("attach-eips")]
        };
        let e = MakeRule {
            target: String::from("sql-runner"),
            recipe: Some(String::from("/opt/sql-runner test")),
            prerequisites: vec![String::from("storage-loader")]
        };

        vec![a, b, c, d, e]
    };

    let makefile = Makefile {
        variables: vars,
        rules: rules,
        dag: Dag::new()
    };

    let expected = String::from("a='bar3'
foo='bar'
z='bar2'

done: sql-runner

emr-etl-runner: 
\t./emr-etl-runner.sh foo bar

attach-eips: 
\t/opt/attach-eips.py test

storage-loader: emr-etl-runner attach-eips
\t./storage-loader.sh alpha beta

sql-runner: storage-loader
\t/opt/sql-runner test

");

    let actual = write_to_string(makefile);
    assert_eq!(actual, expected);
}