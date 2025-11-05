# `std::geo2d`

## `InvoluteGearProfile`

Constructs the profile of an involute gear with a number of teeth, the module size (~teeth size) and a pressure angle.
The profile can be extruded to construct 3D gear.

[![test](.test/std_geo2d_involute_gear_profile.svg)](.test/std_geo2d_involute_gear_profile.log)

```µcad,std_geo2d_involute_gear_profile
std::geo2d::InvoluteGearProfile(module = 4.0mm, teeth = 20, pressure_angle = 0°);
```

## `Pie`

Constructs a point at origin with a radius and a start and end angle.

[![test](.test/std_geo2d_pie.svg)](.test/std_geo2d_pie.log)

```µcad,std_geo2d_pie
std::geo2d::Pie(radius = 20mm, start = 45°, end = 135°);
```

## `RoundedRect`

This constructs a rounded rectangle with a corner radius.

[![test](.test/std_geo2d_rounded_rect.svg)](.test/std_geo2d_rounded_rect.log)

```µcad,std_geo2d_rounded_rect
std::geo2d::RoundedRect(20mm, radius = 5mm);
```

## `Text`

We can construct a text element with `std::geo2d::Text`.
*Dosis* is used as µcad built-in font.

*Note:*
Currently, system fonts cannot be used, we must load fonts directly from a file by settings the `font_file` parameter.
**This API might change in the future.**

[![test](.test/std_geo2d_text.svg)](.test/std_geo2d_text.log)

```µcad,std_geo2d_text
std::geo2d::Text(height = 20mm, text = "Hello µcad!");
```
