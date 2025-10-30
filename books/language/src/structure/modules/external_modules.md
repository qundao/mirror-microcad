# External Modules

External modules are external source files.

For example if you put a second file beside your main source code file, you
can easily import this second file.

[![test](.test/external_modules_main.svg)](.test/external_modules_main.log)

```µcad,external_modules_main
mod second;
second::f(); 
```

[![test](.test/external_modules_second.svg)](.test/external_modules_second.log)

```µcad,external_modules_second
// file: second.µcad
pub fn f() {}
```

By using `mod second,` in the first source file, microcad searches for either a
file called `second.µcad` or `second/mod.µcad` and loads it into a module.
