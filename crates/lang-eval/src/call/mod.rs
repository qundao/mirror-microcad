// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Call related evaluation entities

mod argument_value;
mod argument_value_list;
mod call_method;
mod call_trait;

pub use argument_value::ArgumentValue;
pub use argument_value_list::ArgumentValueList;
pub use call_method::*;
pub use call_trait::*;
