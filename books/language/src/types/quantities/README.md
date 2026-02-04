# Quantities

*Quantities* are numeric values coupled with a unit that refers to a [*quantity type*](types.md).

[![test](.test/quantity_types_number_literals.svg)](.test/quantity_types_number_literals.log)

```µcad,quantity_types_number_literals
// declare variable `height` of type `Length` to 1.4 Meters
height = 1.4m;

// use as *default* value in parameter list
fn f( height = 1m ) {}

// calculate a `Length` called `width` by multiplying the
// `height` with `Scalar` `2` and add ten centimeters
width = height * 2 + 10cm;
```
