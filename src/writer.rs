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

use std::fmt::format;

use types::Makefile;

pub fn write_to_string(make: Makefile) -> String {

    let mut buffer = String::new();

    // Variables
    let mut vars: Vec<String> = make.variables
        .iter()
        .map(|(name, value)| format!("{}='{}'\n", name, value))
        .collect();
    vars.sort();
    for var in vars {
        buffer.push_str(&var);
    }

    buffer.push_str("\n");

    // Rules
    // TODO

    return buffer;
}
