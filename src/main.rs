// Copyright (C) 2017 Oren Ben-Kiki <oren@ben-kiki.org>
//
// This file is part of cargo-fmt.
//
// Digitalis is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License, version 3, as published by the
// Free Software Foundation.
//
// Digitalis is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// cargo-fmt. If not, see <http://www.gnu.org/licenses/>.

//! Ensure source files in a cargo project do not contain `TODOX` issues.

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
extern crate unindent;

#[cfg(test)]
use std::io;
#[cfg(test)]
use std::vec::Vec;
#[cfg(test)]
use unindent::unindent;

// BEGIN NOT TESTED
#[cfg(not(test))]
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
                ),
        )
        .get_matches();
    if let Some(output) = matches
        .subcommand_matches("todox")
        .unwrap()
        .value_of("output")
    {
        let mut file = File::create(output).expect(format!("{}: failed to open", output).as_ref());
        std::process::exit(run(&mut file))
    } else {
        std::process::exit(run(&mut io::stderr()))
    }
}
// END NOT TESTED

fn run(output: &mut Write) -> i32 {
    let ls_files = Command::new("git")
        .arg("ls-files")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute `git ls-files`")
        .wait_with_output()
        .expect("failed to wait for git ls-files");

    if !ls_files.status.success() {
        panic!("git ls-files failed"); // NOT TESTED
    }

    let mut status = 0;
    for path in String::from_utf8(ls_files.stdout).unwrap().lines() {
        if does_file_contain_todox(output, path) {
            status = 1;
        }
    }

    return status;
}

fn does_file_contain_todox(output: &mut Write, path: &str) -> bool {
    let file = File::open(path).expect(format!("{}: failed to open", path).as_ref());
    let mut line_number = 0;
    let mut does_contain_todox = false;
    for line in BufReader::new(file).lines() {
        line_number += 1;
        let text = line.expect(format!("{}:{}: failed to read line", path, line_number).as_ref());
        if !text.contains("ALLOW TODOX") && text.to_lowercase().contains("todox") {
            writeln!(output, "{}:{}: contains todox", path, line_number).unwrap();
            does_contain_todox = true;
        }
    }
    return does_contain_todox;
}

#[cfg(test)]
use std::env;

#[test]
fn test_success() {
    env::set_current_dir("tests/success").unwrap();
    let mut output = io::Cursor::new(Vec::new());
    assert_eq!(run(&mut output), 0);
    assert_eq!(std::str::from_utf8(output.get_ref()).unwrap(), "");
    env::set_current_dir("../..").unwrap();
}

#[test]
fn test_failure() {
    env::set_current_dir("tests/failure").unwrap();
    let mut output = io::Cursor::new(Vec::new());
    assert_eq!(run(&mut output), 1);
    assert_eq!(
        std::str::from_utf8(output.get_ref()).unwrap(),
        unindent(
            r#"
        example.txt:1: contains todox
        example.txt:2: contains todox
        example.txt:3: contains todox
    "#,
        )
    );
    env::set_current_dir("../..").unwrap();
}
