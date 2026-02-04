# Example: use_dome

[![Report](.test/use_dome.svg)](.test/use_dome.log)

```µcad,use_dome
// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod dome;

dome::Dome(10mm, strut_width = 0.5mm).std::ops::mirror(std::math::Z);
```

2D Output
    : ![None](.test/use_dome-out.svg)

3D Output
    : ![None](.test/use_dome-out.stl)
