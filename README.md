# cargo-todox [![Build Status](https://api.travis-ci.org/orenbenkiki/cargo-todox.svg?branch=master)](https://travis-ci.org/orenbenkiki/cargo-todox) [![codecov](https://codecov.io/gh/orenbenkiki/cargo-todox/branch/master/graph/badge.svg)](https://codecov.io/gh/orenbenkiki/cargo-todox) [![docs](https://docs.rs/cargo-todox/badge.svg)](https://docs.rs/cargo-todox)

Ensure source files in a cargo project do not contain `TODOX` issues.

## Installing

To install:

```
cargo install cargo-todox
```

## Running

To run on a cargo project in the current working directory:

```
cargo todox
```

This will list all (case insensitive) occurrences of the string `TODOX` in the
project source files (taken from `git ls-files`). It will succeed if and only if
there are no such occurrences.

Supported command line flags are:
* Run `cargo todox DIRECTORY` to only check files in a specific directory.
* Run `cargo todox --output FILE` to redirect the output to a file.
* Run `cargo todox --version` to report the version number.
* Run `cargo todox --help` to get a help message listing the options.

### Integration with cargo make

If you use `cargo make`, here is one way to integrate `cargo todox` into your
workflow:

```toml
[tasks.todox]  # ALLOW TODOX
description = "Verify the code does not contain leftover TODOX."  # ALLOW TODOX
install_crate = "cargo-todox"  # ALLOW TODOX
command = "cargo"
args = ["todox"]  # ALLOW TODOX

# Verify as part of `cargo make build-flow` and `cargo make ci-flow`.
[tasks.pre-verify-project]
dependencies = [..., "todox"]  # ALLOW TODOX
```

### Checking TODOX on a CI server

To keep your code base clean, it can be helpful to fail the CI build when a pull
request contains leftover `TODOX` issues. To achieve this, include `cargo todox`
in your CI build steps. For example, a minimal Travis setup might look like
this:

```yaml
language: rust
cache: cargo
before_script:
- which cargo-todox || cargo install cargo-todox # ALLOW TODOX
script:
- cargo todox # ALLOW TODOX
- cargo build
- cargo test
```

Note that using `cache: cargo` is optional but highly recommended to speed up
the installation.

## Why use TODOX?

TL;DR: It remembers for you the issues you need to fix before committing,
allowing you to focus on the task at hand.

Note this is very different from tracking normal TODO issues, which should be
handled in future commits.

This is similar to using `report_fixme = "Always"` and using `rustfmt`. The
differences are:

* It looks for the (case insensitive) string `TODOX`, rather than the string
  `FIXME`.

* It looks _everywhere_ in _all_ source files, rather than only in comments in
  `.rs` files.

* It allows for exempting lines by saying `ALLOW TODOX` (typically in a
  comment).

This allows using `todox` as part of the name of a variable, inside comments,
inside `.toml` files, inside `.yaml` files, in the `README` files, etc.

### Without TODOX

You are working on some non-trivial issue, deep in focus. In the process, you
make a temporary change to the code. You make a mental note that you should undo
this before committing. Then you make another. You notice that the comment of
the function you are changing is no longer up to date - in fact, you should also
change the comment of a related class in a different module. You make a mental
note to update both comments before you commit the code. You think it would be a
good idea to add a new test case. You make a mental note to add it before you
commit the code.

You finally get your code to run. The tests run too. It was a long day. You want
to commit it. You just need to fix all these little messes you left behind
first. There was this temporary change, or were there two? And some comments
somewhere? You can't quite recall, you were deep in focus about the code at the
time. It was a long day after all.

You are a conscientious, professional developer. You review `git diff` before
you commit. It is long, since you changed the name of a function called in many
places. You locate and fix some issues. But you have this nagging feeling
there's something you forgot. You commit anyway, hoping that if you forgot
something, you'll remember it tomorrow. If only there was a better way...

### With TODOX

You are working on some non-trivial issue, deep in focus. In the process, you
make a temporary change to the code. You name the temporary variable `todox_foo`
and forget about it. Then you make another such change, you add a `// TODOX:
Revert` comment next to it and forget about it as well. You notice that the
comment of the function you are changing is no longer up to date - in fact, you
should also change the comment of a related class in a different module. You add
a `// TODOX: update comments in here and in X` comment and forget about it. You
think it would be a good idea to add a new test case, you add a `// TODOX: Add
test case for Y` comment anywhere in the code and forget about it as well.

You finally get your code to run. The tests run too. It was a long day. You want
to commit it. You just need to fix all these little messes you left behind. You
run `cargo todox` and get a list of all the issues. You work through them one by
one, either fixing them, or open an issue for fixing them in a later commit. You
commit the code, and go to sleep with a clear conscience. Tomorrow will be a
brand new day.

If, for some reason, you must leave a `TODOX` in your code, include the
incantation `ALLOW TODOX` in the same line (probably in a comment). This will
direct `cargo todox` to ignore the use of `TODOX` in this line.

## License

`cargo-todox` is distributed under the GNU General Public License (Version 3.0).
See the [LICENSE](LICENSE.txt) for details.
