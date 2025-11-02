// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad core geometry traits

/// Trait to align something to center
///
/// TODO: This trait might be extended so that.
pub trait Align<T = Self> {
    /// Align geometry.
    fn align(&self) -> T;
}

/// Return total amount of memory in bytes.
pub trait TotalMemory {
    /// Total amount of memory in bytes.
    fn total_memory(&self) -> usize {
        self.stack_memory() + self.heap_memory()
    }

    /// Get amount of stack memory in bytes.
    fn stack_memory(&self) -> usize {
        std::mem::size_of_val(self)
    }

    /// Get amount of heap memory in bytes.
    fn heap_memory(&self) -> usize {
        0
    }
}

impl<T> TotalMemory for Vec<T> {
    fn heap_memory(&self) -> usize {
        self.capacity() * std::mem::size_of::<T>()
    }
}

/// Return number of vertices.
pub trait VertexCount {
    /// Return vertex count.
    fn vertex_count(&self) -> usize;
}
