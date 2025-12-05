# Match

Having enums leads to wanting a match statement:

```µcad
match align {
    Alignment::FIX => ...,
    Alignment::NEAR => ...,
    Alignment::CENTER => ...,
    Alignment::FAR => ...,
}
```

## Consequences

The match statement alone is a big syntax construct that should be made to make enums most effective.
A check of all enum variants are used within a match might be quite easy with enums but then must also be done with other types.
These must have a possibility to match other cases or different patterns, similar to Rust:

```µcad
a = 1;
match a {
    1 => ...,
    2..3 => ...,
    4.. => ...,
    _ => ...,
}
```
