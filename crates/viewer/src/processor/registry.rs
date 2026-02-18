// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Model Registry.

use bevy::asset::uuid::Uuid;
use microcad_lang::{model::Model, render::ComputedHash};
use rustc_hash::FxHashSet;

const MODEL_UUID_SEED: u64 = 0xDEAD_BEEF_DEED_BEAF;

const MODEL_GEOMETRY_OUTPUT_UUID_SEED: u64 = 0x4321_4321_4321_4321;

/// Generate a Uuid for a model info.
pub fn generate_model_uuid(model: &Model) -> Uuid {
    Uuid::from_u64_pair(MODEL_UUID_SEED, model.as_ptr() as u64)
}

/// Generate a Uuid for a model geometry output.
pub fn generate_model_geometry_output_uuid(model: &Model) -> Uuid {
    Uuid::from_u64_pair(MODEL_GEOMETRY_OUTPUT_UUID_SEED, model.computed_hash())
}

/// The instance registry keeps track of all Uuid that already have been rendered, so we don't have have re-render things.
#[derive(Default)]
pub struct InstanceRegistry {
    /// UUIDs of all geometry outputs (meshes and polygons)
    geometry_output_uuids: FxHashSet<Uuid>,
    /// UUIDs of all model infos.
    model_uuids: FxHashSet<Uuid>,
}

impl InstanceRegistry {
    pub fn contains_geometry_output(&self, uuid: &Uuid) -> bool {
        self.geometry_output_uuids.contains(uuid)
    }

    pub fn contains_model(&self, uuid: &Uuid) -> bool {
        self.model_uuids.contains(uuid)
    }

    pub fn insert_geometry_output(&mut self, uuid: Uuid) {
        self.geometry_output_uuids.insert(uuid);
    }

    pub fn insert_model(&mut self, uuid: Uuid) {
        self.model_uuids.insert(uuid);
    }

    pub fn fetch_model_uuids(&self) -> Vec<Uuid> {
        self.model_uuids.iter().cloned().collect()
    }

    pub fn clear_model_uuids(&mut self) {
        self.model_uuids.clear()
    }
}
