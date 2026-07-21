// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! CastInto Trait

pub trait CastInto<T> {
    fn cast_into(self) -> T;
}

impl<A, B> CastInto<Box<[B]>> for Box<[A]>
where
    A: CastInto<B>,
{
    fn cast_into(self) -> Box<[B]> {
        self.into_iter()
            .map(|a| a.cast_into())
            .collect::<Vec<B>>()
            .into_boxed_slice()
    }
}

impl<A, B> CastInto<Box<B>> for Box<A>
where
    A: CastInto<B>,
    Box<B>: From<Box<A>>,
{
    fn cast_into(self) -> Box<B> {
        self.into()
    }
}

#[macro_export]
macro_rules! impl_cast_into {
    // 1. Identity Implementation (e.g. impl CastInto<Unit> for Unit)
    (
        identity $( $type:ty ),* $(,)?
    ) => {
        $(
            impl crate::CastInto<$type> for $type {
                #[inline]
                fn cast_into(self) -> $type {
                    self
                }
            }
        )*
    };

    // 1. Single-element Tuple Structs (e.g., ListExpression, RangeFirst)
    (
        tuple $type:ident : $kind:tt
    ) => {
        impl<EXPR, T> crate::CastInto<$type<T>> for $type<EXPR>
        where
            EXPR: crate::CastInto<T>,
        {
            fn cast_into(self) -> $type<T> {
                $type(impl_cast_into!(@field self.0, $kind))
            }
        }
    };

    // 2. Structs with Named Fields
    (
        struct $type:ident {
            $( $field:ident : $kind:tt ),* $(,)?
        }
    ) => {
        impl<EXPR, T> crate::CastInto<$type<T>> for $type<EXPR>
        where
            EXPR: crate::CastInto<T>,
        {
            fn cast_into(self) -> $type<T> {
                $type {
                    $(
                        $field: impl_cast_into!(@field self.$field, $kind),
                    )*
                }
            }
        }
    };

    // 3. Single-value Enums
    (
        enum $type:ident {
            $( $variant:ident ),* $(,)?
        }
    ) => {
        impl<EXPR, T> crate::CastInto<$type<T>> for $type<EXPR>
        where
            EXPR: crate::CastInto<T>,
        {
            fn cast_into(self) -> $type<T> {
                match self {
                    $(
                        $type::$variant(inner) => $type::$variant(inner.cast_into()),
                    )*
                }
            }
        }
    };

    // --- Helper Arms ---
    (@field $val:expr, cast)     => { $val.cast_into() };
    (@field $val:expr, box_cast) => { Box::new((*$val).cast_into()) };
    (@field $val:expr, vec_cast) => { $val.into_iter().map(|x| x.cast_into()).collect() };
    (@field $val:expr, keep)     => { $val };
}
