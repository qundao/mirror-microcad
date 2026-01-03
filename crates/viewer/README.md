# microcad viewer

This crate provides the viewer to view µcad files.

## Requirements

On Debian systems using wayland, install the wayland libraries first before compiling:

```µcad
sudo apt install libwayland-dev
```

## Run

Start the microcad viewer by typing `microcad-viewer` into your console.

```sh
microcad-viewer
```

### Run workspace root using Cargo

```sh
RUST_LOG=info cargo run --bin microcad-viewer --package microcad-viewer -- examples/csg_cube.µcad
```
