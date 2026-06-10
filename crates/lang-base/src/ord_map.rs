// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ordered Map

use std::ops::Index;

use microcad_core::hash::HashMap;

/// Trait a value in an `OrdMap` must implement.
/// # Types
/// `K`: key type
pub trait OrdMapValue<K>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    /// return some unique key of this value or `None`
    fn key(&self) -> Option<K>;
}

/// Map whose values can be accessed via index in original insert order.
#[derive(Clone, PartialEq)]
pub struct OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash,
{
    /// vec to store values
    vec: Vec<V>,
    /// map to store key -> index of value in vec
    map: HashMap<K, usize>,
}

impl<K, V> Default for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self {
            vec: Default::default(),
            map: Default::default(),
        }
    }
}

impl<K, V> std::fmt::Debug for OrdMap<K, V>
where
    V: OrdMapValue<K> + std::fmt::Debug,
    K: std::cmp::Eq + std::hash::Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrdMap").field("vec", &self.vec).finish()
    }
}

impl<K, V> From<Vec<V>> for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash,
{
    fn from(vec: Vec<V>) -> Self {
        Self {
            map: vec
                .iter()
                .enumerate()
                .filter_map(|(i, item)| item.key().map(|key| (key, i)))
                .collect(),
            vec,
        }
    }
}

impl<K, V> OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash,
{
    /// get iterator over values in original order
    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.vec.iter()
    }

    /// return number of stored values
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// `true` no values are stored`
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// get value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|index| &self.vec[*index])
    }

    /// get list of all keys
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, usize> {
        self.map.keys()
    }

    /// get first element
    pub fn first(&self) -> Option<&V> {
        self.vec.first()
    }
}

impl<K, V> Index<usize> for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<K, V> Index<&K> for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        &self.vec[*self.map.get(key).expect("key not found")]
    }
}
