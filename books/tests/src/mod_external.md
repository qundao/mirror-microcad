# Load an external module within MD test

[![test](.test/mod_external_root.svg)](.test/mod_external_root.log)

```µcad,mod_external_root
mod external;
std::debug::assert_valid(external::a);
```

[![test](.test/mod_external.svg)](.test/mod_external.log)

```µcad,mod_external
// file: external.µcad
pub const a =1;
```
