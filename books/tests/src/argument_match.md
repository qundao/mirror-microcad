# Argument matching

[![test](.test/argument_match_short_id.svg)](.test/argument_match_short_id.log)

```µcad,argument_match_short_id
fn f( x_param: Length, y_param: Length, z_param=0mm ) -> Volume  {
    x_param * y_param * z_param
}

f( x_param=1m, y_param=2m, z_param=3m );
f( x=1m, y=2m, z=3m );
f( x=1m, y=2m );
```
