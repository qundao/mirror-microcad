# Constant Assignments

Unlike *variable assignments*, constants are not stored on the *stack* but in
the *symbol table*.
For example this allows them to be accessed from within functions or workbenches
in the same module where the constant is defined.
Constants can be placed in
[source files](../structure/source_file.md),
[modules](../structure/modules/) or
[initialization code](../structure/workbenches/elements/init_code.md).

Constants are always written in `UPPER_CASE`.

[![test](.test/const_assignment_mod.svg)](.test/const_assignment_mod.log)

```µcad,const_assignment_mod
const TEXT = "Hello";

mod my_module {
    
    // constant assignment
    const TEXT = "Hello my_module";

    // public function
    pub fn f() -> String {
        TEXT
    }

    // public workbench
    pub sketch MySketch(text: String) {
        std::debug::assert_eq([ TEXT, text ]);
    }
}

my_module::MySketch("Hello my_module");
std::debug::assert_eq([ my_module::f(), "Hello my_module" ]);
std::debug::assert_eq([ TEXT, "Hello" ]);
```

Additionally, constant assignments are permitted in the *init code* of a
*workbench*, where value assignments are likewise prohibited.

[![test](.test/const_assignment_workbench.svg)](.test/const_assignment_workbench.log)

```µcad,const_assignment_workbench
sketch MySketch(text: String) {
    // constant assignment in initialization code
    const TEXT = "Hello";

    init() {
        text = TEXT;
    }

    std::print(text);
}

MySketch();
```

## Rules

### Ambiguous Names

A constant cannot be defined within the same module or workbench twice.

[![test](.test/const_assignment_shadow.svg)](.test/const_assignment_shadow.log)

```µcad,const_assignment_shadow#todo_fail
mod module {
    const A = 5;
    const A = 1;  // error: A already defined in this module

    pub mod another_module {
        const A = 5;   // ok

        pub fn a() -> Integer { A }
    }

    pub sketch Sketch() {
        const A = 5;   // error: A is ambiguous
        const A = 5;   // error: A already defined in this workbench
    }
}

std::debug::assert_eq([ module::another_module::a(), 5 ]);
module::Sketch();
```
