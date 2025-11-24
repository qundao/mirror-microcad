// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io::Cursor;

use bevy::asset::uuid::Uuid;
use bevy::asset::{AssetId, AssetLoader, Assets, Handle, RenderAssetUsages};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Res, ResMut};
use bevy::image::{
    self, CompressedImageFormats, Image, ImageFormat, ImageLoader, ImageSampler, ImageType,
};
use bevy::prelude::{Color, Val};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::scene::ron::de;
use bevy::ui::widget::ImageNode;
use bevy::ui::{BackgroundColor, BorderColor, Node, UiRect};

use crate::State;

#[derive(Component, Default)]
pub struct Overlay {
    frame_entity: Option<Entity>,
    busy_indicator: Option<Entity>,
    //selection_text: Option<Entity>,
}

const IMAGE_UUID_SEED: u64 = 0x1234_1234_4321_4321;

fn generate_image_uuid() -> Uuid {
    Uuid::from_u64_pair(IMAGE_UUID_SEED, IMAGE_UUID_SEED)
}

pub fn setup_overlay(mut images: ResMut<Assets<Image>>) {
    // Your encoded PNG/JPEG bytes
    let encoded: Vec<u8> = include_bytes!("../../assets/icon.png").to_vec();

    // 1. Decode using the `image` crate
    // 1. Decode encoded bytes (PNG/JPEG/etc.)
    let decoded = Image::from_buffer(
        &encoded,
        ImageType::Format(ImageFormat::Png),
        CompressedImageFormats::default(),
        false,
        ImageSampler::Default,
        RenderAssetUsages::default(),
    )
    .expect("No error");

    let size = decoded.size();
    let rgba = decoded.data.unwrap();
    let (width, height) = (size.x, size.y);

    // 2. Create a Bevy `Image`
    let bevy_image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &rgba, // raw RGBA8 pixels
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    // 3. Insert into Assets<Image> with custom UUID
    let handle: Handle<Image> = Handle::Weak(AssetId::Uuid {
        uuid: generate_image_uuid(),
    });

    images.insert(&handle, bevy_image);
}

pub fn update_overlay(
    mut state: ResMut<State>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
) {
    use crate::processor::ProcessingState;

    let processing_state = state.processing_state.clone();
    let overlay = &mut state.scene.overlay;

    match processing_state {
        ProcessingState::Idle => {
            if let Some(frame_entity) = overlay.frame_entity {
                commands.entity(frame_entity).despawn();
                overlay.frame_entity = None;
            }

            if let Some(busy_indicator) = overlay.busy_indicator {
                commands.entity(busy_indicator).despawn();
                overlay.busy_indicator = None;
            }
        }
        ProcessingState::Busy => {
            if let Some(frame_entity) = overlay.frame_entity {
                commands.entity(frame_entity).despawn();
                overlay.frame_entity = None;
            }

            // Spawn a yellow frame.
            if overlay.busy_indicator.is_none() {
                overlay.busy_indicator = Some(
                    commands
                        .spawn((
                            BackgroundColor(Color::NONE),
                            BorderColor(Color::srgb(1.0, 1.0, 0.0)),
                            ImageNode::new(Handle::Weak(AssetId::Uuid {
                                uuid: generate_image_uuid(),
                            })),
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
        ProcessingState::Error => {
            if let Some(busy_indicator) = overlay.busy_indicator {
                commands.entity(busy_indicator).despawn();
                overlay.busy_indicator = None;
            }
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
