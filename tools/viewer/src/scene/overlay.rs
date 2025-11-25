// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use bevy::ecs::component::Component;
use bevy::ecs::query::{With, Without};
use bevy::ecs::system::{Commands, Query, Res};
use bevy::prelude::{Color, Val};
use bevy::render::view::Visibility;
use bevy::ui::{BackgroundColor, BorderColor, Node, UiRect};

use crate::{State, ToBevy};

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct ErrorFrame;

pub fn setup_overlay(state: Res<State>, mut commands: Commands) {
    commands.spawn((
        BackgroundColor(Color::NONE),
        BorderColor(Color::srgb(1.0, 0.0, 0.0)),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        Visibility::Hidden,
        ErrorFrame,
    ));

    commands.spawn((
        BackgroundColor(state.config.theme.brighter.to_bevy()),
        Node {
            position_type: bevy::ui::PositionType::Absolute,
            width: Val::Percent(0.0),
            height: Val::Px(8.0),
            bottom: Val::Px(0.0),
            ..Default::default()
        },
        Visibility::Hidden,
        ProgressBar,
    ));
}

pub fn update_overlay(
    state: Res<State>,
    mut progress_bar: Query<(&mut Node, &mut Visibility), With<ProgressBar>>,
    mut error_frame: Query<(&mut Node, &mut Visibility), (With<ErrorFrame>, Without<ProgressBar>)>,
) {
    use crate::processor::ProcessingState;

    let processing_state = state.processing_state.clone();

    for (mut node, mut visibility) in &mut progress_bar {
        *visibility = match processing_state {
            ProcessingState::Busy(percent) => {
                node.width = Val::Percent(percent);
                Visibility::Visible
            }
            _ => Visibility::Hidden,
        }
    }

    for (_, mut visibility) in &mut error_frame {
        *visibility = if matches!(processing_state, ProcessingState::Error) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
