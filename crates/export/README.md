# µcad export API

[![Crates.io](https://img.shields.io/crates/v/microcad-export.svg)](https://crates.io/crates/microcad-export)
[![Documentation](https://docs.rs/microcad-export/badge.svg)](https://docs.rs/microcad-export/)

This crate provides the µcad export Rust API which provides functionality to export microcad model trees into various file formats.

The following exporters are implemented at the moment:

* `svg`: Export a 2D model as SVG.
* `stl`: Export a 3D model into an STL mesh.
* `ply`: Export a 3D model as PLY.
* `wkt`: Export a 2D model as Well-Known-Text (a simpler format than SVG to represent polygonal geometries).
* `json`: Export any model as JSON.

## ❤️ Support the project

This crate is part of the [microcad project](https://microcad.xyz).

If you like this project, you can help us spend more time on it by donating:

<a href="https://opencollective.com/microcad/donate" target="_blank">
<img src="https://opencollective.com/microcad/donate/button@2x.png?color=blue" width=300 />
</a>
