# Constant Assignments

Unlike *value assignments*, constant assignments are not stored on the *evaluation
stack* but in the *symbol table*.
Constant assignments can be placed directly within a module — which is not allowed
for value assignments — and they can also be declared as public.

[![test](.test/const_assignment_mod.svg)](.test/const_assignment_mod.log)

```µcad,const_assignment_mod
const TEXT = "Hello";

mod my_module {
    pub const TEXT = "Hello";
}

std::print(TEXT);
std::print(my_module::TEXT);
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

Using `pub` is not allowed in workbenches:

[![test](.test/const_assignment_workbench_pub.svg)](.test/const_assignment_workbench_pub.log)

```µcad,const_assignment_workbench_pub#fail
sketch MySketch() {
    pub const TEXT = "Hello";  // error
    std::print(TEXT);
}

MySketch();
```
