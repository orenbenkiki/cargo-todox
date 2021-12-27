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
#[macro_use]
extern crate clap;

#[cfg(not(test))]
use clap::{App, AppSettings, Arg, SubCommand};
#[cfg(not(test))]
use std::io;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

#[cfg(test)]
use std::io;

#[cfg(test)]
use std::vec::Vec;

#[cfg(test)]
use unindent::unindent;

// TODO: Should be "not tested"
#[cfg(not(test))]
#[doc(hidden)]
fn main() {
    let matches = App::new("cargo")
        .bin_name("cargo")
        .version(crate_version!())
        .about("Ensure source files in a cargo project do not contain TODOX issues.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("todox")
                .about("Scan current working directory for TODOX.")
                .version(crate_version!())
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Redirect output to a file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("directory")
                        .help("the directory containing the project files (by default, '.')")
                        .required(false),
                ),
        )
        .get_matches();

    let directory = matches
        .value_of("directory")
        .map_or(".", |argument| argument);

    let status = matches
        .subcommand_matches("todox")
        .unwrap()
        .value_of("output")
        .map_or(run(&mut io::stderr(), directory), |output| {
            let mut file =
                File::create(output).unwrap_or_else(|_| panic!("{}: failed to open", output));
            run(&mut file, directory)
        });

    std::process::exit(status);
}

#[doc(hidden)]
fn run(output: &mut dyn Write, directory: &str) -> i32 {
    let ls_files = Command::new("git")
        .arg("ls-files")
        .arg(directory)
        .stdout(Stdio::piped())
        .spawn()
        .expect(&("failed to execute `git ls-files` ".to_owned() + directory))
        .wait_with_output()
        .expect(&("failed to wait for git ls-files ".to_owned() + directory));

    if !ls_files.status.success() {
        panic!("git ls-files failed"); // NOT TESTED
    }

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
    let file = File::open(path).unwrap_or_else(|_| panic!("{}: failed to open", path));
    let mut does_contain_todox = false;
    for (mut line_number, line) in BufReader::new(file).lines().enumerate() {
        line_number += 1;
        let text = line.unwrap_or_else(|_| panic!("{}:{}: failed to read line", path, line_number));
        if !text.contains("ALLOW TODOX") && text.to_lowercase().contains("todox") {
            writeln!(output, "{}:{}: contains todox", path, line_number).unwrap();
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
