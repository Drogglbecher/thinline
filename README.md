<p align="center">
    <img src="./.github/logo.png">
</p>
<p align="center">
    <a href="https://travis-ci.org/Drogglbecher/thinline"><img alt="Travis Status" src="https://travis-ci.org/Drogglbecher/thinline.svg"></a>
    <a href="https://ci.appveyor.com/project/Drogglbecher/thinline"><img alt="Appveyor Status" src="https://ci.appveyor.com/api/projects/status/r3ldomvr2plv336t/branch/master?svg=true"></a>
    <a href="https://coveralls.io/github/Drogglbecher/thinline?branch=master"><img alt="Test Coverage" src="https://coveralls.io/repos/github/Drogglbecher/thinline/badge.svg?branch=master"></a>
    <a href="https://drogglbecher.github.io/thinline"><img alt="Documentation" src="https://img.shields.io/badge/master_doc-thinline-blue.svg"></a>
    <a href="https://crates.io/crates/thinline"><img alt="Crates.io" src="https://img.shields.io/crates/v/thinline.svg"></a>
    <a href="https://github.com/Drogglbecher/thinline/blob/master/LICENSE"><img alt="License Apache 2" src="https://img.shields.io/badge/license-Apache%202-blue.svg"></a>
</p>

## Description

_Thinline_ is a project for handling and executing unittests written in function comment sections for C/C++ & Python.
**It's currently under development, analysis works partially but the synthesis part won't work right now.**

## Installation

### Requirements

To use _thinline_ you need a valid [Rust](https://www.rust-lang.org/en-US/install.html) installation and its package
manager [cargo](https://crates.io/install). Depending on your OS you can install them via the package manager you
like the most. Besides this you can use [rustup](https://rustup.rs/) if you want but keep in mind that this can
conflict with already existing installations of rust, so uninstall them first.

Besides this _thinline_ uses the [rust clang implementation](https://github.com/KyleMayes/clang-rs), so please make
sure to also fulfill its [requirements](https://github.com/KyleMayes/clang-sys#dependencies).

### crates.io

_Thinline_ is available on [crates.io](https://crates.io/), the official rust crate registry.

```
cargo install thinline
```

### Manual installation

Just clone this repo and then this simple installation command should be enough:

```
cargo install
```

## CLI-Usage

The usage of the CLI-tool is basically simple, `thinline --help` prints the usage:

```
USAGE:
    thinline [FLAGS] [OPTIONS] <SOURCE-DIR> --language <LANGUAGE>

FLAGS:
    -b, --build      Executes the build script steps given in the project thinline setting file.
    -d, --dry-run    Creates only the test files in the target projects `.thinline` folder without exexcuting them.
    -h, --help       Prints help information
    -q, --quiet      Does not print any console logs.
    -V, --version    Prints version information
    -v               Set the verbosity level (`v` -> DEBUG, `vv` -> TRACE).

OPTIONS:
    -l, --language <LANGUAGE>           Specifies the language of the target project. [possible values: c,
                                        cpp, python]
    -p, --project-config <YAML_FILE>    The name of the yaml file where the project parameters for thinline are stored.
                                        This file has to be at <SOURCE-DIR>. [default: .thinline.yml]

ARGS:
    <SOURCE-DIR>    The directory where the sources for test-extraction are located
```
