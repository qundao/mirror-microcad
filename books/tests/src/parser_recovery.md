# Various error recovery in the parser

[![test](.test/parser_recovery_wrong_fn_params.svg)](.test/parser_recovery_wrong_fn_params.log)

```µcad,parser_recovery_wrong_fn_params#fail
fn bar (a: 1, b: mod) { // error, invalid params
  1=; // error inside an if statement with broken params
}

a =; // error after an if statement with broken params
```

[![test](.test/parser_recovery_missing_fn_params.svg)](.test/parser_recovery_missing_fn_params.log)

```µcad,parser_recovery_missing_fn_params#fail
fn bar { // error, no params
  1=; // error inside an if statement with missing params
}

a =; // error after an if statement with missing params
```

[![test](.test/parser_recovery_missing_mod_name.svg)](.test/parser_recovery_missing_mod_name.log)

```µcad,parser_recovery_missing_mod_name#fail
mod { // error, no name
  1=; // error inside a mod statements with no name
}

a =; // error after a mod statements with no name
```

[![test](.test/parser_recovery_return_extras.svg)](.test/parser_recovery_return_extras.log)

```µcad,parser_recovery_return_extras#fail
return 1 2;// error, unexpected tokens after return statement

a =; // error after a broken return statement
```

[![test](.test/parser_recovery_use_non_identifier.svg)](.test/parser_recovery_use_non_identifier.log)

```µcad,parser_recovery_use_non_identifier#fail
use a::1; // error, non-identifier in use name

a =; // error after a broken use statement
```

[![test](.test/parser_recovery_use_missing_as.svg)](.test/parser_recovery_use_missing_as.log)

```µcad,parser_recovery_use_missing_as#fail
use a::b as ; // error, missing as name

a =; // error after a broken use statement
```

[![test](.test/parser_recovery_use_invalid_as.svg)](.test/parser_recovery_use_invalid_as.log)

```µcad,parser_recovery_use_invalid_as#fail
use a::b as 1; // error, invalid as name

a =; // error after a broken use statement
```

[![test](.test/parser_recovery_use_leading_double_colon.svg)](.test/parser_recovery_use_leading_double_colon.log)

```µcad,parser_recovery_use_leading_double_colon#fail
use a:: as 1; // error, name

a =; // error after a broken use statement
```