# µcad language

- [Program Structure](#program-structure)
- [Data Types](#data-types)
- [Calls](#calls)
- [Objects](#objects)
- [Libraries](#libraries)

The *µcad programming language* is purely declarative, which means that a µcad program can be
evaluated like a mathematical equation, resulting in a graphical output.
It only needs to know the values of all the variables to obtain this result.

## Program Structure

A **µcad program** can simply consist of a sequence of [*statements*](doc/structure/statements.md)  in a
*source file* or more complex constructs such as [*workbenches*](doc/structure/workbench.md) and
[*functions*](doc/structure/functions.md).

Additionally, [*modules*](doc/structure/modules.md) help bundle things into packages and resolve naming
collision issues.

With [*use statements*](doc/structure/use.md) functionalities of a module can be used in other modules.

## Data Types

µcad knows several [*primitive types*](doc/types/primitive_types.md) (like `String` and `Integer`)
and [*quantity types*](doc/types/quantity.md) which are always linked to a *unit* (like `Length` in
`mm` or an `Angle` in `°`).

[*Collections*](doc/types/collections.md) (like [*tuples*](doc/types/tuples.md) or [*arrays*](doc/types/arrays.md))
can bundle other types into structured sets.

Produced 3D and 2D objects are stored in [*nodes*](doc/types/nodes.md).

It's planned to implement [*custom types*](doc/types/custom_types.md) in future.

## Calls

In µcad you may [call](doc/structure/calls.md) [*workbenches*](doc/structure/calls.md#calling-workbenches) to produce
*objects* from [sketches](doc/structure/sketch.md) or [*parts*](doc/structure/part.md) or you may call
[*functions*](doc/structure/calls.md#calling-functions).

In both cases you shall read about [*argument multiplicity*](doc/structure/arguments.md#argument-multiplicity) and
[*argument matching*](doc/structure/arguments.md#argument-matching) to understand how µcad is processing arguments.

## Objects

The 2D or 3D objects produced with [*workbenches*](doc/structure/calls.md#calling-workbenches) can be
[measured](doc/nodes/measures.md) or [exported](doc/attributes/export.md).

## Libraries

µcad knows two kinds of libraries: Those which are written µcad language and those which
are written in *Rust*.

The [standard library](doc/libs/std/README.md) is a µcad library which encapsulates the
[builtin library](doc/libs/builtin/README.md) into a nice and convenient interface.

If you want to make your own libraries put you µcad code into the search paths or create
a [plugin](doc/libs/plugins.md) to embed Rust code.

## Debugging

There are several builtin [debug functionalities](doc/debug/README.md) which help you debugging your code.
