# microcad

[![Crates.io](https://img.shields.io/crates/v/microcad.svg)](https://crates.io/crates/microcad)
[![Documentation](https://docs.rs/microcad/badge.svg)](https://docs.rs/microcad/)

This crate provides a command line interface for µcad language.

**Note**: This project is in an early stage of development and is not yet feature complete!

## Quick Start

To process a µcad file and export its geometry into an STL, use the following command:

```sh
microcad export ./examples/bricks/brick
```

This will export the 3D geometry description in `./examples/bricks/brick.µcad` as a `brick.stl` file.

## Installation

First, install [CMake](https://cmake.org/) and [Ninja](https://github.com/ninja-build/ninja) which are needed to compile
the [manifold geometry library](https://github.com/elalish/manifold).

### *Debian* based *Linux* distributions

Use the following line to install Ninja and CMake:

```sh
sudo apt install ninja-build cmake
```

Pre-built debian packages which are attached to our release logs:

### MacOS

If you are on using MacOS, you have to install `cmake` and `ninja` using `brew`:

```sh
brew install cmake ninja
```

### Install µcad via cargo

To install the latest release of *µcad* via *cargo*, type:

```sh
cargo install microcad
```

## Run

Start the microcad CLI by typing `microcad` into your console.

```sh
microcad
```

This will give you an overview of the available functionality:

```plain
µcad Command Line Interface

Usage: microcad [OPTIONS] <COMMAND>

Commands:
  parse        Parse a µcad file
  resolve      Parse and resolve a µcad file
  eval         Parse and evaluate a µcad file
  export       Parse and evaluate and export a µcad file
  create       Create a new source file with µcad extension
  watch        Watch a µcad file
  install      Install µcad standard library
  completions  Print shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
  -T, --time                  Display processing time
  -C, --config <CONFIG_PATH>  Load config from file
  -v...                       Verbosity level (use -v, -vv, or -vvv)
  -h, --help                  Print help
  -V, --version               Print version
```

You're now ready to use µcad!

## Command line usage

To generate an STL model file, use the `export` command (with an optional output file name):

```sh
microcad export ./examples/bricks/brick
```

This will export the geometry into a `brick.stl` file.

## Resources

* Documentation: <https://docs.microcad.xyz/language/book/index.html>
* µcad Viewer: <https://crates.io/crates/microcad-viewer/>
*

## ❤️ Support the project

This crate is part of the [microcad project](https://microcad.xyz).

If you like this project, you can help us spend more time on it by donating:

<a href="https://opencollective.com/microcad/donate" target="_blank">
<img src="https://opencollective.com/microcad/donate/button@2x.png?color=blue" width=300 />
</a>
