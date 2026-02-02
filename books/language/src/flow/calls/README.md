# Calls

*Workbenches* and *functions* can get called, which just means there inner code
gets executed.
There are several types of calls which have some different effects or usage.

| Type           | Example                           | Input(s)                    | Output       |
| -------------- | --------------------------------- | --------------------------- | ------------ |
| function call  | `value = my_function(..);`        | parameter list              | Value        |
| workbench call | `model = MySketch(..);`           | parameter list              | Model[^prop] |
| operation call | `new_model = model.my_operation;` | parameter list<br>and Model | Model[^prop] |

[^prop]: including properties.

## Calling Functions

A call of a [function](../../structure/functions/)  consists of just the *identifier* and an [argument list](arguments.md).
and the result is a *value*:

[![test](.test/call_function.svg)](.test/call_function.log)

```µcad,call_function
// function definition
fn square(x: Scalar) { return x * x; }

// call function square with parameter 2 and store result in s
s = square(x = 2);

// check value
std::debug::assert_eq( [s, 4] );
```

## Calling Workbenches

[Workbenches](workbench.md) can be called in the same way as functions
except that the result is a *model*.

[![test](.test/call_workbench.svg)](.test/call_workbench.log)

```µcad,call_workbench
// definition of a sketch workbench
sketch Square(size: Length) { 
    std::geo2d::Rect(size);
}

// call square with a size and store object node in s
s = Square(size=2cm);

// translate object s
s.std::ops::translate(x = 1cm);
```

## Calling Operations

[Operations](op.md) are called differently.
