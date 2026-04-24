use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};

use crate::{
    ApngAsset, ApngDespawnable, ApngNode, ApngPlayer, ApngSprite, messages::ApngDespawnMessage,
};

pub(crate) fn initialize_gifs(
    apng_q: Query<(
        Option<(&ApngSprite, &mut Sprite)>,
        Option<(&ApngNode, &mut ImageNode)>,
        &mut ApngPlayer,
    )>,
    mut apngs: ResMut<Assets<ApngAsset>>,
    asset_server: ResMut<AssetServer>,
) {
    for (sprite_opt, node_opt, mut player) in apng_q {
        let handle = if let Some((sprite, _)) = sprite_opt {
            sprite.handle.clone()
        } else if let Some((node, _)) = node_opt {
            node.handle.clone()
        } else {
            panic!("a apng player was inserted unexpectedly");
        };

        if let Some(ApngAsset {
            frames,
            handles,
            times,
        }) = apngs.get_mut(&handle)
        {
            if !handles.is_empty() {
                continue;
            }

            for frame in frames.iter() {
                let image = Image::new_fill(
                    Extent3d {
                        width: frame.width,
                        height: frame.height,
                        depth_or_array_layers: 1,
                    },
                    bevy::render::render_resource::TextureDimension::D2,
                    &frame.pixels,
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::all(),
                );
                let handle = asset_server.add(image);
                handles.push(handle);
            }
            let frame = frames.first().unwrap();
            let handle = handles.first().unwrap();

            if let Some((_, mut sprite)) = sprite_opt {
                sprite.image = handle.clone();
            }
            if let Some((_, mut node)) = node_opt {
                node.image = handle.clone();
            }

            player.current = 0;
            player.timer = Timer::new(frame.duration, TimerMode::Repeating);
            player.remaining = *times;
        }
    }
}

pub(crate) fn animate_apngs(
    apng_q: Query<(
        Option<(&ApngSprite, &mut Sprite)>,
        Option<(&ApngNode, &mut ImageNode)>,
        &mut ApngPlayer,
    )>,
    apngs: Res<Assets<ApngAsset>>,
    time: Res<Time>,
    mut writer: MessageWriter<ApngDespawnMessage>,
) {
    for (sprite_opt, node_opt, mut player) in apng_q {
        let handle = if let Some((sprite, _)) = sprite_opt {
            sprite.handle.clone()
        } else if let Some((node, _)) = node_opt {
            node.handle.clone()
        } else {
            panic!("a apng player was inserted unexpectedly");
        };

        if let Some(apng_asset) = apngs.get(&handle) {
            player.timer.tick(time.delta());
            if player.timer.is_finished() {
                player.current = (player.current + 1) % apng_asset.frames.len();
                let frame = &apng_asset.frames[player.current];
                let new_duration = frame.duration;

                if player.current == 0
                    && let Some(remaining) = player.remaining
                {
                    if remaining == 0 {
                        player.timer.pause();
                        writer.write(ApngDespawnMessage(handle.clone()));
                    } else {
                        player.remaining = Some(remaining);
                    }
                }

                player.timer.set_duration(new_duration);
                player.timer.reset();

                let handle = apng_asset.handles[player.current].clone();
                if let Some((_, mut sprite)) = sprite_opt {
                    sprite.image = handle.clone();
                }
                if let Some((_, mut node)) = node_opt {
                    node.image = handle.clone();
                }
            }
        }
    }
}

pub(crate) fn despawn_apngs(
    mut commands: Commands,
    apng_q: Query<(Option<&ApngSprite>, Option<&ApngNode>, Entity), With<ApngDespawnable>>,
    mut reader: MessageReader<ApngDespawnMessage>,
) {
    for ApngDespawnMessage(handle) in reader.read() {
        for (sprite_opt, node_opt, entity) in apng_q {
            let target_handle = if let Some(sprite) = sprite_opt {
                sprite.handle.clone()
            } else if let Some(node) = node_opt {
                node.handle.clone()
            } else {
                panic!("a apng player was inserted unexpectedly");
            };

            if target_handle.id() == handle.id() {
                commands.entity(entity).despawn();
            }
        }
    }
}
