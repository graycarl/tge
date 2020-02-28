use crate::error::{GameError, GameResult};
use crate::math::Size;
use crate::engine::Engine;
use std::path::Path;

pub struct Image {
    size: Size<u32>,
    pixels: Vec<u8>,
}

impl Image {

    pub fn new<S: Into<Size<u32>>>(size: S, pixels: Vec<u8>) -> GameResult<Self> {
        let size = size.into();
        validate_pixels(size, &pixels)?;
        Ok(Self { size, pixels })
    }

    pub fn from_bytes(bytes: &[u8]) -> GameResult<Self> {
        let image = image::load_from_memory(bytes)
            .map_err(|error| GameError::InitError(Box::new(error)))?
            .into_rgba();
        let size = Size::new(image.width(), image.height());
        let pixels = image.into_raw();
        Self::new(size, pixels)
    }

    pub fn load<P: AsRef<Path>>(engine: &mut Engine, path: P) -> GameResult<Self> {
        let bytes = engine.filesystem().read(path)?;
        Self::from_bytes(&bytes)
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn into_pixels(self) -> Vec<u8> {
        self.pixels
    }

}

pub fn validate_pixels(size: Size<u32>, pixels: &[u8]) -> GameResult {
    if (size.width * size.height * 4) as usize == pixels.len() {
        Ok(())
    } else {
        Err(GameError::RuntimeError("illegal pixels length".into()))
    }
}
