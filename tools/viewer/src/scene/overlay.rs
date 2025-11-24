// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use bevy::ecs::system::Commands;
use bevy::prelude::{Color, Val};
use bevy::text::{TextColor, TextLayout};
use bevy::ui::widget::Text;
use bevy::ui::{BackgroundColor, BorderColor, Node, PositionType, UiRect};

struct Overlay {
    color: Color,
}

pub fn spawn_overlay(mut commands: Commands) {
    // UI root node (a transparent full-screen container)
    commands.spawn((
        BackgroundColor(Color::NONE),
        BorderColor(Color::srgb(1.0, 0.0, 0.0)),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
    ));

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
