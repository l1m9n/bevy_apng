use std::time::Duration;

use bevy::{asset::AssetLoader, prelude::*};
use png::Decoder;
use thiserror::Error;

#[derive(Component, Debug, Clone)]
pub struct ApngPlayer {
    pub current: usize,
    pub timer: Timer,
    pub remaining: Option<u32>,
}

impl Default for ApngPlayer {
    fn default() -> Self {
        Self {
            current: 0,
            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            remaining: None,
        }
    }
}

#[derive(Component, Debug, Clone)]
#[require(Sprite, ApngPlayer)]
pub struct ApngSprite {
    pub handle: Handle<ApngAsset>,
}

#[derive(Component, Debug, Clone)]
#[require(ImageNode, ApngPlayer)]
pub struct ApngNode {
    pub handle: Handle<ApngAsset>,
}

#[derive(Component)]
pub struct ApngDespawnable;

#[derive(Debug, Asset, TypePath, Clone)]
pub struct ApngAsset {
    pub frames: Vec<ApngFrame>,
    pub handles: Vec<Handle<Image>>,
    pub times: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ApngFrame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub duration: Duration,
}

#[derive(Default, Debug, TypePath)]
pub struct ApngLoader;

#[derive(Error, Debug)]
pub enum ApngLoaderError {
    #[error("could not load asset: {0}")]
    Io(#[from] std::io::Error),

    #[error("could not decode asset: {0}")]
    Decode(#[from] png::DecodingError)
}

impl AssetLoader for ApngLoader {
    type Asset = ApngAsset;
    type Settings = ();
    type Error = ApngLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> std::result::Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let decoder = Decoder::new(std::io::Cursor::new(bytes));
        let mut decoder = decoder.read_info()?;

        let mut frames = Vec::new();
        let mut buf = vec![0; decoder.output_buffer_size().unwrap()];
        while let Ok(info) = decoder.next_frame(&mut buf) {
            let width = info.width;
            let height = info.height;
            let bytes = buf[..info.buffer_size()].to_vec();

            let frame_control = &decoder.info().frame_control.unwrap();

            let ms = (frame_control.delay_num as f32) / (frame_control.delay_den as f32);
            let duration = Duration::from_secs_f32(ms.max(1.));

            frames.push(ApngFrame {
                width,
                height,
                pixels: bytes,
                duration,
            });
        }

        let times = decoder.info().animation_control.unwrap().num_plays;
        let mut times_opt = None;
        if times != 0 {
            times_opt = Some(times);
        }
        let times = times_opt;

        let asset = ApngAsset {
            frames,
            handles: vec![],
            times,
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["gif"]
    }
}
