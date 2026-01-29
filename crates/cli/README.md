# microcad

This crate provides the command line interpreter of µcad.

**Note**: This project is in an early stage of development and is not yet feature complete!

## Installation

First, install [CMake](https://cmake.org/) and [Ninja](https://github.com/ninja-build/ninja) which are needed to compile
the [manifold geometry library](https://github.com/elalish/manifold).

### *Debian* based *Linux* distributions

Use the following line to install Ninja and CMake:

```sh
sudo apt install ninja-build cmake
```

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

## Command line usage

After installing, you can run a basic example by typing:

```sh
microcad eval ./examples/bricks/brick
```

This will *evaluate* the input file and will output the model tree.
The *evaluate* command will not export the output geometry.

To generate an STL model file use the `export` command with an additional output file name:

```sh
microcad export ./examples/bricks/brick
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
  parse    Parse a µcad file
  resolve  Parse and resolve a µcad file
  eval     Parse and evaluate a µcad file
  export   Parse and evaluate and export a µcad file
  create   Create a new source file with µcad extension
  watch    Watch a µcad file
  install  Install µcad standard library
  help     Print this message or the help of the given subcommand(s)

Options:
  -T, --time             Display processing time
  -C, --config <CONFIG>  Load config from file
  -v...                  Verbosity level (use -v, -vv, or -vvv)
  -h, --help             Print help
  -V, --version          Print version
```

## Install standard library

In most cases you might want to use the *µcad standard library* (`std`).

This must be installed once by running the following command:

```sh
microcad install std
```

You're now ready to use µcad!

## Resources

- Tutorials:

  - <http://microcad.xyz>  
- Language documentation:

  - <https://docs.microcad.xyz/language/book/index.html>

## ❤️ Donate

If you like this project, you can help us spend more time on it by donating:

<a href="https://opencollective.com/microcad/donate" target="_blank">
<img src="https://opencollective.com/microcad/donate/button@2x.png?color=blue" width=300 />
</a>
