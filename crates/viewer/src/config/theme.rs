// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Color theme

use bevy::ecs::resource::Resource;
use microcad_core::Color;
use serde::Deserialize;

/// Represents a color theme.
#[derive(Resource, Clone, Debug, PartialEq, Deserialize)]
pub struct Theme {
    /// Name of the theme.
    pub name: Option<String>,

    /// Primary color.
    #[serde(deserialize_with = "deserialize_color_from_hex")]
    pub primary: Color,

    /// Brighter color.
    #[serde(deserialize_with = "deserialize_color_from_hex")]
    pub bright: Color,

    /// Brighter color.
    #[serde(deserialize_with = "deserialize_color_from_hex")]
    pub brighter: Color,

    /// Darker color.
    #[serde(deserialize_with = "deserialize_color_from_hex")]
    pub darker: Color,

    /// Complementary color (e.g. operators, intersections).
    #[serde(deserialize_with = "deserialize_color_from_hex")]
    pub complementary: Color,

    /// Signal color, e.g. for selections.
    #[serde(deserialize_with = "deserialize_color_from_hex")]
    pub signal: Color,

    /// Color for guidelines (e.g. inference lines).
    #[serde(deserialize_with = "deserialize_color_from_hex")]
    pub guide: Color,
}

fn deserialize_color_from_hex<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Color::from_hex_str(&s)
        .map_err(|err| serde::de::Error::custom(format!("invalid color '{}': {}", s, err)))
}

impl Default for Theme {
    fn default() -> Self {
        let hex = |s| Color::from_hex_str(s).expect("Valid hex string");

        Self {
            name: Some("default".into()),
            primary: hex("#001728"),
            bright: hex("#DBF3FF"),
            brighter: hex("#7CA0B1"),
            darker: hex("#477991"),
            complementary: hex("#7D0303"),
            signal: hex("#FFA41C"),
            guide: hex("#009CDE"),
        }
    }
}
