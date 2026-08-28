// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL exporter.

use microcad_lang_types::{ModelNodeRef, ModelType, Value};

use crate::{ExportError, Exporter, ExporterParameters};

/// STL Exporter.
pub struct StlExporter;

impl Exporter for StlExporter {
    fn export<'tree>(
        &self,
        _model: &ModelNodeRef<'tree>,
        _parameters: &ExporterParameters,
    ) -> Result<Value, ExportError> {
        todo!()
        /*
        use crate stl::{StlWriter, WriteStl};
        let mut f = std::fs::File::create(parameters.path)?;
        let mut writer = StlWriter::new(&mut f)?;

        model.write_stl(&mut writer)?;
        Ok(Value::None)*/
    }

    fn model_type(&self) -> ModelType {
        ModelType::Geometry3D
    }
}
