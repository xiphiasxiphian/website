use std::{collections::HashMap, path::{Path, PathBuf}};

use image::ImageError;
use wgpu::{BindGroupLayout, Device, Queue};

use crate::renderer::texture::Texture;

#[derive(Default)]
pub struct AssetPool
{
    textures: HashMap<PathBuf, Texture>
}

impl AssetPool
{
    fn get_texture(
        &mut self,
        path: &Path,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> Result<&Texture, ImageError>
    {
        if !self.textures.contains_key(path)
        {
            let texture = Texture::from_path(path, device, queue, layout)?;
            self.textures.insert(path.to_path_buf(), texture);
        }

        Ok(
            self.textures
            .get(path)
            .expect("Unreachable")
        )
    }
}
