# Constant Assignments

Unlike *value assignments*, constant assignments are not stored on the *evaluation
stack* but in the *symbol table*.
Constant assignments can be placed directly within a module — which is not allowed
for value assignments — and they can also be declared as public.

[![test](.test/const_assignment_mod.svg)](.test/const_assignment_mod.log)

```µcad,const_assignment_mod
const TEXT = "Hello";

mod my_module {
    // (private) constant
    const TEXT = "Hello my_module";

    // public function
    pub fn f() -> String{
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
    const TEXT = "Hello";

    init() {
        text = TEXT;
    }

    std::print(text);
}

MySketch();
```
