# Example: torus

[![test](.test/torus.svg)](.test/torus.log)

```µcad,torus
// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

std::geo3d::Torus([1..5] * 10mm, minor_radius = 3mm);

```

![test](.test/torus-out.svg)

![test](.test/torus-out.stl)
