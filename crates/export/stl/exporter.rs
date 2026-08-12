// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL exporter.

use microcad_lang_types::{ModelOutputType, ModelRef, Value};

use crate::{ExportError, Exporter, ExporterParameters};

/// STL Exporter.
pub struct StlExporter;

impl<'tree> Exporter<'tree> for StlExporter {
    fn export(
        &self,
        _model: &ModelRef<'tree>,
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

    fn output_type(&self) -> ModelOutputType {
        ModelOutputType::Geometry3D
    }
}
