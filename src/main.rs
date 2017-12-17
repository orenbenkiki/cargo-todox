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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() {
    if std::env::args().count() > 2 {
        print!("cargo todox takes no arguments.\n");
        std::process::exit(1);
    }

    let output = Command::new("git")
        .arg("ls-files")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute `git ls-files`")
        .wait_with_output()
        .expect("failed to wait for git ls-files");

    if !output.status.success() {
        panic!("git ls-files failed");
    }

    let mut status = 0;
    for path in String::from_utf8(output.stdout).unwrap().lines() {
        if does_file_contain_todox(path) {
            status = 1;
        }
    }

    std::process::exit(status);
}

fn does_file_contain_todox(path: &str) -> bool {
    match File::open(path) {
        Err(error) => {
            print!("{}: {}\n", path, error);
            return false;
        }
        Ok(file) => {
            let mut line_number = 0;
            let mut does_contain_todox = false;
            for line in BufReader::new(file).lines() {
                line_number += 1;
                match line {
                    Err(error) => {
                        print!("{}:{}: {}\n", path, line_number, error);
                    }
                    Ok(text) => {
                        if !text.contains("ALLOW TODOX") && text.to_lowercase().contains("todox") {
                            print!("{}:{}: contains todox\n", path, line_number);
                            does_contain_todox = true;
                        }
                    }
                }
            }
            return does_contain_todox;
        }
    }
}
