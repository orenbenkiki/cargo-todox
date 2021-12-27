# cargo-todox

[![Verify pushed commit](https://github.com/orenbenkiki/cargo-todox/actions/workflows/on_push.yml/badge.svg)](https://github.com/orenbenkiki/cargo-todox/actions/workflows/on_push.yml) [![Monthly audit](https://github.com/orenbenkiki/cargo-todox/actions/workflows/monthly_audit.yml/badge.svg)](https://github.com/orenbenkiki/cargo-todox/actions/workflows/on_updated_dependencies.yml) [![codecov](https://codecov.io/gh/orenbenkiki/cargo-todox/branch/master/graph/badge.svg)](https://codecov.io/gh/orenbenkiki/cargo-todox) [![Api Docs](https://docs.rs/cargo-todox/badge.svg)](https://docs.rs/crate/cargo-todox)

Ensure source files in a cargo project do not contain `TODOX` issues.

## Installing

To install:

```console
cargo install cargo-todox
```

## Running

To run on a cargo project in the current working directory:

```console
cargo todox
```

This will list all (case insensitive) occurrences of the string `TODOX` _anywhere_ in _any_ of the project source files
(taken from `git ls-files`). It will succeed if and only if there are no such occurrences.

Supported command line flags are:
* Run `cargo todox DIRECTORY` to only check files in a specific directory.
* Run `cargo todox --output FILE` to redirect the output to a file.
* Run `cargo todox --version` to report the version number.
* Run `cargo todox --help` to get a help message listing the options.

## Why use TODOX?

Using `TODOX` allows making temporary changes to the code which must be addressed before checking the code in. This can
for any of several reasons: debugging, getting something quick-and-dirty running, investigating alternatives, or just
noting an issue which must be addressed.

Whatever the reason, using `TODOX` allows marking the issue and moving on without having to worry about it. Before
completing the work, all such markers need to be removed, either by converting them into a `TODO` for later work or by
fixing the issue. This is where `cargo todox` comes in - it emits a nicely formatted list of all the locations of this
marker anywhere in the project, and only exits with a zero status only if no such markers are found. It is therefore
easy to include `cargo todox` in the pre-commit checks, and/or in CI builds.

This is a generalization of using `report_fixme = "Always"` in `rustfmt`, a feature which seems on its way to be
[removed](https://github.com/rust-lang/rustfmt/issues/5102). The differences between `report_fixme` and `cargo todox`
are:

* It looks _everywhere_ in _all_ source files, rather than only in comments in `.rs` files. This allows using `todox` as
  a part of the name of a variable, inside comments, inside printed debugging messages, inside `.toml` files, inside
  `.yaml` files, in the `README` files, etc. In contrast, `FIXME` is only detected in comments in rust files, which
  allows for most, but not all, of the useful cases for such a marker.

* It allows for exempting lines by saying `ALLOW TODOX` (typically in a comment). This allows configuration files to
  specify running `cargo todox` without being reported themselved.

* It looks for the (case insensitive) string `TODOX`, rather than the string `FIXME`.

## License

`cargo-todox` is distributed under the GNU General Public License (Version 3.0). See the [LICENSE](LICENSE.txt) for
details.