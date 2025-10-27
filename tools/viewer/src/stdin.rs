// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer Standard input interface.

use std::io::BufRead;

use crossbeam::channel::Receiver;

use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        resource::Resource,
        system::{Query, Res, ResMut},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SourceLocation {
    line: u64,
    col: u64,
    source_hash: u64,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum StdinMessage {
    CursorPosition(SourceLocation),
    ChangeColor { r: f32, g: f32, b: f32 },
}

#[derive(Resource)]
pub struct MessageReceiver(Receiver<StdinMessage>);

impl MessageReceiver {
    pub fn run() -> Self {
        // Create channel for stdin reader to communicate with Bevy
        let (sender, receiver) = crossbeam::channel::unbounded();

        // Spawn thread to read from stdin
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines().map_while(Result::ok) {
                match serde_json::from_str::<crate::stdin::StdinMessage>(&line) {
                    Ok(msg) => {
                        if sender.send(msg).is_err() {
                            break;
                        }
                    }
                    Err(e) => eprintln!("Invalid input: {e}"),
                }
            }
        });

        Self(receiver)
    }
}

/// Message handler
pub fn handle_messages(
    receiver: Res<MessageReceiver>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    for msg in receiver.0.try_iter() {
        match msg {
            StdinMessage::ChangeColor { r, g, b } => {
                println!("Message received: {r} {g} {b}");

                for mat in query.iter_mut() {
                    if let Some(material) = materials.get_mut(&mat.0) {
                        material.base_color = Color::srgb(r, g, b);
                    }
                }
            }
            StdinMessage::CursorPosition(_) => todo!("Handle cursor position"),
        }
    }
}
