# Language Design

- [µcad Rules](#µcad-rules)
- [The Build Process](#the-build-process)
  - [Parsing Phase](#parsing-phase)
  - [Resolving Phase](#resolving-phase)
  - [Evaluation Phase](#evaluation-phase)
  - [Rendering](#rendering)
  - [Export Phase](#export-phase)

## µcad Rules

To stay on course while continuing development of µcad we have expressed some rules that may guide us:

1. µcad is a declarative markup language. This means µcad avoids imperative control flow statements like `for`, `while` or `goto`.
2. Complex generic functionalities (like generating objects or mathematical functionalities)
   shall be put into *builtin libraries* instead of implementing them within µcad itself.
3. The *standard library* shall implement a convenient interface to the builtin library.
4. µcad enforces a strict *modular concept* to improve code quality.
5. Explicit naming of *arguments* (within in *calls*) is encouraged.
6. Explicit writing of *units* at *values* is forced.

## The Build Process

The *µcad* interpreter runs programs which generate geometry files.
The processing of *µcad* source code files into output files can be divided into separate phases:

### Parsing Phase

In the parsing phase one source file is read into a *syntax tree* by using the [*µcad* parser](../syntax).
Any errors which occur within the parsing phase are related to file access or syntax.

### Resolving Phase

In the resolving phase, the following steps are done:

1. the search paths will be recursively scanned for any external µcad files
2. all symbols (e,g, parts, functions, constants and modules) in the source file will be put into a symbol tree

At the end all dependent files are loaded and syntax definitions are placed in the symbol tree and everything can be found by it's qualified name.

### Evaluation Phase

In the evaluation phase, the *syntax tree*  will be processed into a *model tree*
which is a structured representation of the geometry.
While this phase the following things will be done:

- expressions will be calculated
- functions will be called
- parts will generate *model nodes*
- user messages will be output on console

Any errors which occur within the evaluation phase are related to semantic issues.

### Rendering

A rendered (like an STL or SVG) renders the model tree into its specific representation, like triangle meshes or polygons.

### Export Phase

In the export phase, the renderers representation will be taken to generate 2D or 3D output files
(e.g. *SVG* or *STL*).
While this phase the following things will be done:

- geometric operations will be processed
- geometries will be rendered
- the output files will be written

Any errors which occur within the export phase are related to geometrical processing or file access.
