// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mathematical operations.

pub trait MathOps {
    type Output;
    type Error;

    fn sqrt(&self) -> Result<Self::Output, Self::Error>;
    fn abs(&self) -> Result<Self::Output, Self::Error>;
    fn sin(&self) -> Result<Self::Output, Self::Error>;
    fn cos(&self) -> Result<Self::Output, Self::Error>;
    fn tan(&self) -> Result<Self::Output, Self::Error>;
}
