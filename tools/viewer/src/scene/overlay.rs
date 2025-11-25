// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::prelude::{Color, Val};
use bevy::render::view::Visibility;
use bevy::ui::{BackgroundColor, BorderColor, Node, UiRect};

use crate::{State, ToBevy};

#[derive(Component)]
pub struct BusyIndicator;

#[derive(Component, Default)]
pub struct Overlay {
    frame_entity: Option<Entity>,
    //selection_text: Option<Entity>,
}

pub fn setup_overlay(state: Res<State>, mut commands: Commands) {
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
        BusyIndicator,
    ));
}

pub fn update_overlay(
    mut state: ResMut<State>,
    mut commands: Commands,
    mut busy_indicator: Query<(&mut Node, &mut Visibility), With<BusyIndicator>>,
) {
    use crate::processor::ProcessingState;

    let processing_state = state.processing_state.clone();
    let overlay = &mut state.scene.overlay;

    for (_, mut visibility) in &mut busy_indicator {
        *visibility = if matches!(processing_state, ProcessingState::Busy(_)) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    match processing_state {
        ProcessingState::Idle => {
            if let Some(frame_entity) = overlay.frame_entity {
                commands.entity(frame_entity).despawn();
                overlay.frame_entity = None;
            }
        }
        ProcessingState::Busy(percent) => {
            if let Some(frame_entity) = overlay.frame_entity {
                commands.entity(frame_entity).despawn();
                overlay.frame_entity = None;
            }

            for (mut node, mut visibility) in &mut busy_indicator {
                *visibility = Visibility::Visible;
                node.width = Val::Percent(percent);
            }
        }
        ProcessingState::Error => {
            // Spawn a red frame.
            if overlay.frame_entity.is_none() {
                overlay.frame_entity = Some(
                    commands
                        .spawn((
                            BackgroundColor(Color::NONE),
                            BorderColor(Color::srgb(1.0, 0.0, 0.0)),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                border: UiRect::all(Val::Px(2.)),
                                ..Default::default()
                            },
                        ))
                        .id(),
                );
            }
        }
    }

    /*
    // Render text.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..Default::default()
        },
        Text::new("ring_1"),
        TextColor(Color::WHITE),
        TextLayout::new_with_no_wrap(),
    ));*/
}
