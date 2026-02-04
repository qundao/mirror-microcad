# Example: ngons

[![test](.test/ngons.svg)](.test/ngons.log)

```µcad,ngons
// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::geo2d::*;
use std::ops::*;

(Hexagon(20mm) - Ngon(5, 10mm)).extrude(20mm);

```

![test](.test/ngons-out.svg)

![test](.test/ngons-out.stl)
