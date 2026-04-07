# Code blocks

## Empty code block

```µcad,test_empty
```

## Code block with failing test

```µcad,test_fail#fail
Circle(r = 42mm);
```

## Code block with parameters

```µcad,test_params(hires)
Rect(r = 42mm);
```

## Multi-line Code block

[![test](.test/multi_line.svg)](.test/multi_line.log)

```µcad,multi_line
use std::print;

print("Hello, µcad standard library!");
```
