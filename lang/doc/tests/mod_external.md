# Load an external module within MD test

```µcad,mod_external_root
mod external;
std::debug::assert_valid(external::a);
```

```µcad,mod_external
// file: external.µcad
pub const a =1;
```
