# µcad export API

This crate provides the µcad export API which provides functionality to export µcad model trees into various file formats.

The following exporters are implemented at the moment:

* `svg`: Export a 2D model as SVG.
* `stl`: Export a 3D model into an STL mesh.
* `ply`: Export a 3D model as PLY.
* `wkt`: Export a 2D model as Well-Known-Text (a simpler format than SVG to represent polygonal geometries).
* `json`: Export any model as JSON.
