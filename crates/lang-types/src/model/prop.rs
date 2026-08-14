// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model properties type.

use microcad_lang_base::{Identifier, SrcRef};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Deref;

use crate::{Arguments, Ty, Value};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// An input property, created from workbenches base parameters.
    Input,
    /// An output property, created by each assignment with the `prop` keyword.
    Output,
    /// A hidden property that can be displayed in the viewer, but is not accessible.
    Hidden,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Property {
    pub name: Identifier,
    pub value: Value,
    pub src_ref: SrcRef,
    pub ty: PropertyType,
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Choose key icon based on visibility
        let ty = match self.ty {
            PropertyType::Hidden => "     ",
            PropertyType::Input => "[in] ",
            PropertyType::Output => "[out]",
        };

        // Render as: 🔑 .radius: Length = 5mm
        write!(
            f,
            "{ty} .{}: {} = {}",
            self.name,
            self.value.ty(),
            self.value
        )
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Properties {
    props: BTreeMap<Identifier, Property>,
}

impl Properties {
    /// Creates a new, empty set of properties.
    pub fn new() -> Self {
        Self {
            props: BTreeMap::new(),
        }
    }

    /// Gets a reference to a property by its name.
    pub fn get_property(&self, name: impl Into<Identifier>) -> Option<&Property> {
        self.props.get(&name.into())
    }

    /// Sets or updates a property.
    ///
    /// If the property already exists, its value, visibility, and source reference are updated.
    /// Returns a reference to the newly inserted or updated [`Property`].
    pub fn set_property(
        &mut self,
        name: impl Into<Identifier>,
        value: impl Into<Value>,
        ty: PropertyType,
        src_ref: SrcRef,
    ) -> &Property {
        let name = name.into();
        let property = Property {
            name: name.clone(),
            value: value.into(),
            src_ref,
            ty,
        };
        self.props.insert(name.clone(), property);
        self.props.get(&name).expect("Property was just inserted")
    }

    /// Returns `true` if a property with the given name exists.
    pub fn contains(&self, name: impl Into<Identifier>) -> bool {
        self.props.contains_key(&name.into())
    }

    /// Returns the number of properties.
    pub fn len(&self) -> usize {
        self.props.len()
    }

    /// Returns `true` if there are no properties set.
    pub fn is_empty(&self) -> bool {
        self.props.is_empty()
    }
}

// --- Ergonomics Implementations ---

impl Deref for Properties {
    type Target = BTreeMap<Identifier, Property>;

    fn deref(&self) -> &Self::Target {
        &self.props
    }
}

impl IntoIterator for Properties {
    type Item = (Identifier, Property);
    type IntoIter = std::collections::btree_map::IntoIter<Identifier, Property>;

    fn into_iter(self) -> Self::IntoIter {
        self.props.into_iter()
    }
}

impl<'a> IntoIterator for &'a Properties {
    type Item = (&'a Identifier, &'a Property);
    type IntoIter = std::collections::btree_map::Iter<'a, Identifier, Property>;

    fn into_iter(self) -> Self::IntoIter {
        self.props.iter()
    }
}

impl From<Arguments> for Properties {
    fn from(args: Arguments) -> Self {
        let mut properties = Properties::new();

        for (name, value) in args.named_iter() {
            properties.set_property(
                name.clone(),
                value.clone(),
                PropertyType::Input,
                SrcRef::default(),
            );
        }

        properties
    }
}
