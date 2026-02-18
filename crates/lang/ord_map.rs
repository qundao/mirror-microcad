// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ordered Map

use std::ops::Index;

/// Trait a value in an `OrdMap` must implement.
/// # Types
/// `K`: key type
pub trait OrdMapValue<K>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// return some unique key of this value or `None`
    fn key(&self) -> Option<K>;
}

/// Map whose values can be accessed via index in original insert order.
#[derive(Clone, PartialEq)]
pub struct OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// vec to store values
    vec: Vec<V>,
    /// map to store key -> index of value in vec
    map: std::collections::HashMap<K, usize>,
}

impl<K, V> Default for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
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
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrdMap").field("vec", &self.vec).finish()
    }
}

impl<K, V> From<Vec<V>> for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn from(vec: Vec<V>) -> Self {
        let mut map = std::collections::HashMap::new();
        // TODO remove for loop use for_each and filter
        for (i, item) in vec.iter().enumerate() {
            if let Some(key) = item.key() {
                map.insert(key, i);
            }
        }

        Self { vec, map }
    }
}

struct OrdMapIterator<'a, K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    ord_map: &'a OrdMap<K, V>,
    iter: std::collections::hash_map::Iter<'a, K, usize>,
}

impl<'a, K, V> Iterator for OrdMapIterator<'a, K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(n) => Some((n.0, &self.ord_map.vec[*n.1])),
            None => None,
        }
    }
}

impl<'a, K, V> OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// get iterator over values in original order
    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.vec.iter()
    }

    /// get iterator over values from map
    pub fn map_iter(&'a self) -> OrdMapIterator<'a, K, V> {
        OrdMapIterator {
            ord_map: &self,
            iter: self.map.iter(),
        }
    }

    /// return number of stored values
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// `true` no values are stored`
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// add new value
    ///
    /// On duplicated named entries, it returns the existing key and the new, duplicate key
    pub fn try_push(&mut self, item: V) -> Result<(), (K, K)> {
        if let Some(key) = item.key() {
            match self.map.entry(key) {
                std::collections::hash_map::Entry::Vacant(entry) => entry.insert(self.vec.len()),
                std::collections::hash_map::Entry::Occupied(entry) => {
                    return Err((entry.key().clone(), item.key().unwrap()));
                }
            };
        }
        self.vec.push(item);
        Ok(())
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
