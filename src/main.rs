// Copyright (C) 2017-2021 Oren Ben-Kiki <oren@ben-kiki.org>
//
// This file is part of cargo-todox.
//
// cargo-todox is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License, version 3, as published by the
// Free Software Foundation.
//
// cargo-todox is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// cargo-todox. If not, see <http://www.gnu.org/licenses/>.

#![doc = include_str!("../README.md")]
#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

#[cfg(not(test))]
use clap::{command, Arg, Command as ClapCommand};

#[cfg(not(test))]
use std::io;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command as ProcessCommand, Stdio};

#[cfg(test)]
use std::io;

#[cfg(test)]
use unindent::unindent;

/// The current crate version: 0.2.5-dev
pub const VERSION: &str = "0.2.5-dev";

#[cfg(not(test))]
#[doc(hidden)]
fn main() {
    let matches = command!()
        .bin_name("cargo")
        .version(VERSION)
        .about("Ensure source files in a cargo project do not contain TODOX issues.")
        .subcommand_required(true)
        .subcommand(
            ClapCommand::new("todox")
                .about("Scan current working directory for TODOX.")
                .version(VERSION)
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Redirect output to a file")
                        .num_args(1),
                )
                .arg(
                    Arg::new("directory")
                        .help("the directory containing the project files (by default, '.')")
                        .required(false), // FLAKY TESTED
                ), // FLAKY TESTED
        )
        .get_matches(); // FLAKY TESTED

    let directory = matches
        .get_one::<String>("directory") // FLAKY TESTED
        .map_or(".", |argument| argument);

    let status = matches // FLAKY TESTED
        .subcommand_matches("todox")
        .unwrap()
        .get_one::<String>("output") // FLAKY TESTED
        .map_or_else(|| run(&mut io::stderr(), directory), |output| { // FLAKY TESTED
            let mut file = // FLAKY TESTED
                File::create(output).unwrap_or_else(|_| panic!("{output}: failed to open")); // FLAKY TESTED
            run(&mut file, directory) // FLAKY TESTED
        });

    std::process::exit(status);
} // FLAKY TESTED

#[doc(hidden)]
fn run(output: &mut dyn Write, directory: &str) -> i32 {
    let ls_files = ProcessCommand::new("git")
        .arg("ls-files")
        .arg(directory)
        .stdout(Stdio::piped())
        .spawn()
        .expect(&("failed to execute `git ls-files` ".to_owned() + directory))
        .wait_with_output()
        .expect(&("failed to wait for git ls-files ".to_owned() + directory));

    assert!(ls_files.status.success(), "git ls-files failed");

    let mut status = 0;
    for path in String::from_utf8(ls_files.stdout).unwrap().lines() {
        if does_file_contain_todox(output, path) {
            status = 1;
        }
    }

    status
}

#[doc(hidden)]
fn does_file_contain_todox(output: &mut dyn Write, path: &str) -> bool {
    let file = File::open(path).unwrap_or_else(|_| panic!("{path}: failed to open"));
    let mut does_contain_todox = false;
    for (mut line_number, line) in BufReader::new(file).lines().enumerate() {
        line_number += 1;
        let text = line.unwrap_or_else(|_| panic!("{path}:{line_number}: failed to read line"));
        if !text.contains("ALLOW TODOX") && text.to_lowercase().contains("todox") {
            writeln!(output, "{path}:{line_number}: contains todox").unwrap();
            does_contain_todox = true;
        }
    }
    does_contain_todox
}

#[test]
fn test_success() {
    let mut output = io::Cursor::new(Vec::new());
    assert_eq!(run(&mut output, "tests/success"), 0);
    assert_eq!(std::str::from_utf8(output.get_ref()).unwrap(), "");
}

#[test]
fn test_failure() {
    let mut output = io::Cursor::new(Vec::new());
    assert_eq!(run(&mut output, "tests/failure"), 1);
    assert_eq!(
        std::str::from_utf8(output.get_ref()).unwrap(),
        unindent(
            r#"
        tests/failure/example.txt:1: contains todox
        tests/failure/example.txt:2: contains todox
        tests/failure/example.txt:3: contains todox
    "#,
        )
    );
}
