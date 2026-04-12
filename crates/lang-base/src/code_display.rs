// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Default code format context
#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct CodeFormatContext<'a> {
    /// Current indentation depth
    pub depth: usize,
    /// Formatter.
    #[deref]
    #[deref_mut]
    pub f: std::fmt::Formatter<'a>,
}

impl<'a> CodeFormatContext<'a> {
    /// Write indent
    pub fn indent(&mut self) -> std::fmt::Result {
        let indent = self.depth * 4;
        write!(self, "{:indent$}", "")
    }

    pub fn nest<F>(&mut self, f: F) -> std::fmt::Result
    where
        F: FnOnce(&mut Self) -> std::fmt::Result,
    {
        self.depth += 1;
        let result = f(self);
        self.depth -= 1;
        result
    }
}

#[macro_export]
macro_rules! indented {
    ($dst:expr, $($arg:tt)*) => {
        {
            $dst.indent()?;
            write!($dst, $($arg)*)
        }
    };
}

/// Trait to display valid µcad source code.
pub trait CodeDisplay<'a, Context = CodeFormatContext<'a>> {
    /// Display µcad source code.
    fn code_display(&self, f: &mut Context) -> std::fmt::Result;
}

/// If it can be Displayed, it can be CodeDisplayed.
impl<'a, T> CodeDisplay<'a> for T
where
    T: std::fmt::Display,
{
    fn code_display(&self, ctx: &mut CodeFormatContext) -> std::fmt::Result {
        write!(ctx, "{self}")
    }
}

/// For statements or items that should be on their own indented lines
pub struct CodeStack<'a, T>(pub &'a [T]);

/// For inline items separated by commas (like function arguments)
pub struct CodeList<'a, T>(pub &'a [T]);

impl<'a, 'b, T> CodeDisplay<'a> for CodeList<'b, T>
where
    T: CodeDisplay<'a>,
{
    fn code_display(&self, f: &mut CodeFormatContext<'a>) -> std::fmt::Result {
        self.0.iter().try_for_each(|item| {
            item.code_display(f)?;
            write!(f, ", ")
        })
    }
}

impl<'a, 'b, T> CodeDisplay<'a> for CodeStack<'b, T>
where
    T: CodeDisplay<'a>,
{
    fn code_display(&self, f: &mut CodeFormatContext<'a>) -> std::fmt::Result {
        self.0.iter().try_for_each(|item| {
            item.code_display(f)?;
            writeln!(f)
        })
    }
}

#[macro_export]
macro_rules! code_display {
    // Overload 2: The Body (Vertical/Braced)
    // We open the brace, nest the formatter, and loop through the exprs
    ($f:expr => { $($body:expr)* }) => {
        {
            writeln!($f, "{{")?;
            $f.nest(|f| {
                $(
                    $body.code_display(f)?;
                    writeln!(f)?;
                )*
                Ok(())
            })?;
            microcad_lang_base::indented!($f, "}}")
        }
    };


    // Overload 1: The Header (Horizontal/Inline formatting)
    // Matches: f => "fmt", args...
    ($f:expr => $($arg:expr)*) => {
        {
            $(
                $arg.code_display($f)?;
            )*
            Ok(())
        }
    };

}
