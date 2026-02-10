# Symbols

```µcad,symbol_doublet#fail
mod foo {
    pub const A = 1mm;
}

fn foo() -> Length { // error: Ambiguous identifiers
    foo::A
}

foo();
```
