use super::{opengl, Filter, Texture, TextureHolder};
use crate::error::{GameError, GameResult};
use crate::math::{Size, Region};
use crate::engine::Engine;
use fontdue::FontSettings;
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub enum CacheBy {
    Add(Region),
    Exist(Region),
}

pub enum CacheError {
    TooLarge,
    NoRoom,
}

struct Inner {
    cache_size: u32,
    texture: Texture,
    uvs: HashMap<(char, usize, usize), Region>,
    rows: Vec<Size<u32>>,
}

pub struct Font {
    font: fontdue::Font,
    inner: RefCell<Inner>,
}

impl Font {

    pub(crate) fn new(engine: &mut Engine, bytes: &[u8], cache_size: u32) -> GameResult<Self> {
        let font = fontdue::Font::from_bytes(bytes, FontSettings::default())
            .map_err(|error| GameError::InitError(error.into()))?;
        let texture = Texture::for_font_cache(engine, cache_size)?;
        let uvs = HashMap::new();
        let rows = Vec::new();
        Ok(Self {
            font,
            inner: RefCell::new(Inner {
                cache_size,
                texture,
                uvs,
                rows,
            })
        })
    }

    pub fn from_bytes(engine: &mut Engine, bytes: &[u8]) -> GameResult<Self> {
        Self::new(engine, bytes, 1024)
    }

    pub fn load(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<Self> {
        let bytes = engine.filesystem().read(path)?;
        Self::from_bytes(engine, &bytes)
    }

    pub(crate) fn font(&self) -> &fontdue::Font {
        &self.font
    }

    pub(crate) fn cache_size(&self) -> u32 {
        self.inner.borrow().cache_size
    }

    pub(crate) fn clone_texture(&self) -> Rc<opengl::Texture> {
        self.inner.borrow().texture.texture().clone()
    }

    pub(crate) fn cache_glyph(&self, character: char, px: f32) -> Result<CacheBy, CacheError> {
        let mut inner = self.inner.borrow_mut();
        let metrics = self.font.metrics(character, px);
        let cache_key = (character, metrics.width, metrics.height);
        if let Some(uv) = inner.uvs.get(&cache_key) {
            return Ok(CacheBy::Exist(*uv));
        }
        let cache_size = inner.cache_size;
        let glyph_size = Size::new(metrics.width as u32, metrics.height as u32);
        let glyph_cache_size = Size::new(glyph_size.width + 1, glyph_size.height + 1);
        if glyph_cache_size.width > cache_size || glyph_cache_size.height > cache_size {
            return Err(CacheError::TooLarge);
        }
        let mut region = None;
        let mut row_bottom = 0;
        for row in inner.rows.iter_mut() {
            if glyph_cache_size.height <= row.height && glyph_cache_size.width <= cache_size - row.width {
                region = Some(Region::new(row.width, row_bottom, glyph_size.width, glyph_size.height));
                row.width += glyph_cache_size.width;
                break;
            }
            row_bottom += row.height;
        }
        if region.is_none() {
            if glyph_cache_size.height <= cache_size - row_bottom {
                region = Some(Region::new(0, row_bottom, glyph_size.width, glyph_size.height));
                inner.rows.push(glyph_cache_size);
            }
        }
        if let Some(region) = region {
            let (_, bitmap) = self.font.rasterize(character, px);
            let mut pixels = Vec::with_capacity(bitmap.len() * 4);
            for alpha in bitmap {
                pixels.push(255);
                pixels.push(255);
                pixels.push(255);
                pixels.push(alpha);
            }
            inner.texture.update_pixels(region, Some(&pixels))
                .expect("update font cache texture error");
            let texture_size = {
                let texture_size = inner.texture.size();
                Size::new(texture_size.width as f32, texture_size.height as f32)
            };
            let uv = Region::new(
                region.x as f32 / texture_size.width,
                region.y as f32 / texture_size.height,
                region.width as f32 / texture_size.width,
                region.height as f32 / texture_size.height,
            );
            inner.uvs.insert(cache_key, uv);
            Ok(CacheBy::Add(uv))
        } else {
            Err(CacheError::NoRoom)
        }
    }

    pub(crate) fn clear_cache(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.uvs.clear();
        inner.rows.clear();
    }

    pub(crate) fn resize_cache(&self, cache_size: u32) {
        let mut inner = self.inner.borrow_mut();
        inner.uvs.clear();
        inner.rows.clear();
        inner.texture.init_pixels((cache_size, cache_size), None)
            .expect("resize font cache texture error");
        inner.cache_size = cache_size;
    }

    pub fn filter(&self) -> Filter {
        self.inner.borrow().texture.filter()
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.inner.borrow_mut().texture.set_filter(filter)
    }

}
