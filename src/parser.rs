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

use nom::{IResult,not_line_ending, space, is_alphanumeric, multispace, ErrorKind};
use nom::Err::Position;

//use std::str;
use std::str::from_utf8;
use std::collections::HashMap;

/// Recognizes numerical and alphabetic characters: 0-9a-zA-Z, plus _
fn variable_name(input:&[u8]) -> IResult<&[u8], &[u8]> {
  for (idx, item) in input.iter().enumerate() {
    if !is_alphanumeric(*item) && *item != '_' as u8 {
      if idx == 0 {
        return IResult::Error(Position(ErrorKind::AlphaNumeric, input))
      } else {
        return IResult::Done(&input[idx..], &input[0..idx])
      }
    }
  }
  IResult::Done(b"", input)
}

named!(key_value    <&[u8],(&str,&str)>,
  chain!(
    key: map_res!(variable_name, from_utf8) ~
         space?                            ~
         tag!("=")                         ~
         space?                            ~
    val: map_res!(
           take_until_either!("\n;"),
           from_utf8
         )                                 ~
         space?                            ~
         chain!(
           tag!(";")        ~
           not_line_ending  ,
           ||{}
         ) ?                               ~
         multispace?                       ,
    ||{(key, val)}
  )
);

named!(keys_and_values_aggregator<&[u8], Vec<(&str,&str)> >, many0!(key_value));

pub fn keys_and_values(input:&[u8]) -> IResult<&[u8], HashMap<&str, &str> > {
  let mut h: HashMap<&str, &str> = HashMap::new();

  match keys_and_values_aggregator(input) {
    IResult::Done(i,tuple_vec) => {
      for &(k,v) in &tuple_vec {
        h.insert(k, v);
      }
      IResult::Done(i, h)
    },
    IResult::Incomplete(a)     => IResult::Incomplete(a),
    IResult::Error(a)          => IResult::Error(a)
  }
}
