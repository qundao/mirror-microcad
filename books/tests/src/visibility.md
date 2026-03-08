# Visibility

[![test](.test/visibility.svg)](.test/visibility.log)

```µcad,visibility
mod my {
    mod mod_private {
        pub const_public = 1;
        const const_private = 1;
    }
    pub mod mod_public {
        pub const_public = 1;
        const const_private = 1;
    }
}

use __builtin::debug::assert;
use __builtin::debug::is_valid;


assert(is_valid("my::mod_public::const_public"));
assert(!is_valid("my::mod_public::const_private"));
assert(!is_valid("my::private_public::const_public"));
assert(!is_valid("my::private_public::const_private"));
```
