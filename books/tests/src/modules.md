# Modules

[![test](.test/builtin_modules.svg)](.test/builtin_modules.log)

```µcad,builtin_modules
mod a {
    pub mod b {
        pub mod c {
            pub part M1() { std::geo3d::Sphere(radius=1cm) }
        }
    }

    pub part M2() { std::geo3d::Sphere(radius=2cm) }
}

a::b::c::M1();
a::M2();
```
