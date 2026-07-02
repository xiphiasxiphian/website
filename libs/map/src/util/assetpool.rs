use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use image::ImageError;
use wgpu::{BindGroupLayout, Device, Queue};

use crate::renderer::texture::Texture;

pub type TextureAsset = Arc<Texture>;

#[derive(Default)]
pub struct AssetPool
{
    textures: HashMap<PathBuf, TextureAsset>,
}

impl AssetPool
{
    // maybe have some preloading here later
    pub fn new() -> Self
    {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get_texture(
        &mut self,
        path: &Path,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> Result<TextureAsset, AssetPoolError>
    {
        let path_buf = path.to_path_buf().canonicalize()?;
        if !self.textures.contains_key(&path_buf)
        {
            let texture = Texture::from_path(path, device, queue, layout)?;
            self.textures.insert(path_buf.clone(), Arc::new(texture));
        }

        Ok(Arc::clone(&self.textures[&path_buf]))
    }
}

#[derive(Debug)]
pub enum AssetPoolError
{
    IOError(std::io::Error),
    ImageError(ImageError),
}

impl From<ImageError> for AssetPoolError
{
    fn from(value: ImageError) -> Self
    {
        Self::ImageError(value)
    }
}

impl From<std::io::Error> for AssetPoolError
{
    fn from(value: std::io::Error) -> Self
    {
        Self::IOError(value)
    }
}
