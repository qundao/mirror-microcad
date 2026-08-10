// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::{Deref, DerefMut, Display, From};
use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_proc_macros::SrcReferrer;

use crate::{Tuple, Ty, Type, Value, ValueError, ValueResult};

#[derive(Clone, Display, From, Debug, PartialEq)]
pub struct Arguments(pub Tuple);

impl Arguments {
    pub fn positional_iter(&self) -> impl Iterator<Item = &Value> {
        self.0.positional.iter()
    }

    pub fn named_iter(&self) -> impl Iterator<Item = (&Identifier, &Value)> {
        self.0.named.iter().map(|(k, v)| (k, v))
    }

    /// Retrieves a named [`Value`] by key.
    ///
    /// # Panics
    ///
    /// Panics if the argument key is not present.
    #[track_caller]
    pub fn get(&self, name: &str) -> &Value {
        self.named_iter()
            .find_map(|(id, v)| {
                if id == &Identifier::no_ref(name) {
                    Some(v)
                } else {
                    None
                }
            })
            .unwrap()
    }

    /// Get `lhs` and `rhs` from binary arguments.
    pub fn get_binary(self) -> (Value, Value) {
        (self.get("lhs").clone(), self.get("rhs").clone())
    }

    pub fn get_unary(self) -> Value {
        self.get("rhs").clone()
    }

    pub fn get_cond(&self) -> ValueResult<bool> {
        self.get_as("cond")
    }

    pub fn get_as<T: TryFrom<Value, Error = ValueError>>(&self, name: &str) -> ValueResult<T> {
        Ok(T::try_from(self.get(name).clone())?)
    }
}

/// Argument value.
#[derive(Clone, Debug)]
pub struct ArgumentValue {
    /// *value* of the argument.
    pub value: Value,
    /// If expression of value is a single identifier, this item catches it.
    pub id: Option<Identifier>,
}

impl std::fmt::Display for ArgumentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{val}", val = self.value,)
    }
}

impl Ty for ArgumentValue {
    fn ty(&self) -> Type {
        self.value.ty()
    }
}

impl ArgumentValue {
    /// Create new argument value
    pub fn new(value: Value, id: Option<Identifier>) -> Self {
        Self { value, id }
    }
}

/// Collection of *argument values* (e.g. `( x=1, y=2 )`).
///
/// Also provides methods to find a matching call
/// between it and a given *parameter list*.
#[derive(Clone, Debug, Default, Deref, DerefMut, SrcReferrer)]
pub struct ArgumentValueList {
    #[deref]
    #[deref_mut]
    pub args: Vec<ArgumentValue>,
    pub src_ref: SrcRef,
}

impl std::fmt::Display for ArgumentValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })
    }
}

impl<Arg> FromIterator<Arg> for ArgumentValueList
where
    Arg: Into<ArgumentValue>,
{
    fn from_iter<T: IntoIterator<Item = Arg>>(iter: T) -> Self {
        let args: Vec<_> = iter.into_iter().map(|a| a.into()).collect();
        Self {
            src_ref: SrcRef::none(),
            args,
        }
    }
}

/// Create arguments via macro
/// arguments!(a = Length::mm(10.0))
#[macro_export]
macro_rules! arguments {
    ($($key:ident = $value:expr),*) => {
            microcad_lang_types::Arguments::from(microcad_lang_types::Tuple::from([$( (stringify!($key), microcad_lang_types::Value::from($value)) ),* ]
                .iter()))
    };
}
