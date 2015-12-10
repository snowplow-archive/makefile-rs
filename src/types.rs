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

use std::collections::HashMap;

use daggy::Dag;

pub type MakeVariables = HashMap<String, String>;

pub struct MakeRule {
    // Rule name
    name: String,
    // Shell command for rule
    command: String
}

pub type MakeDag = Dag<MakeRule, u32, u32>;

pub struct Makefile {
    // Variables
	variables: MakeVariables,
    // DAG of rules
    dag: MakeDag
} 

