// PROPRIETARY AND CONFIDENTIAL
//
// Unauthorized copying of this file via any medium is strictly prohibited.
//
// Copyright (c) 2015 Snowplow Analytics Ltd. All rights reserved.

extern crate getopts;

use getopts::Options;
use std::env;

use std::process::Command;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} JOB VERSION [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "configs", "job name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output = Command::new("sh")
                         .arg("-c")
                         .arg("echo hello")
                         .output()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let hello = output.stdout;

}
