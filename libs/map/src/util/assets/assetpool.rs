use std::{collections::HashMap, sync::Arc};

use image::ImageError;
use wgpu::{BindGroupLayout, Device, Queue};

use crate::renderer::texture::Texture;

pub type Asset<T> = Arc<T>;
pub type TextureAsset = Asset<Texture>;

#[derive(Default)]
pub struct AssetPool
{
    textures: HashMap<&'static str, TextureAsset>,
}

impl AssetPool
{
    pub fn preloaded(
        textures: &[(&'static str, &[u8])],
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> Result<Self, AssetPoolError>
    {
        let mut pool = Self::default();

        for (name, bytes) in textures
        {
            let texture = Texture::from_bytes(bytes, device, queue, layout)?;
            pool.textures.insert(name, Arc::new(texture));
            log::info!("Loaded texture: {}", name);
        }

        Ok(pool)
    }

    pub fn get_texture(&self, id: &'static str) -> Result<TextureAsset, AssetPoolError>
    {
        self.textures
            .get(id)
            .map(Arc::clone)
            .ok_or(AssetPoolError::NotFound(id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssetPoolError
{
    #[error("Texture '{0}' doesn't exist in pool")]
    NotFound(&'static str),
    #[error("Image decode error: {0}")]
    ImageError(#[from] ImageError),
}
