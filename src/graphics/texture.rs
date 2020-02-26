use super::{opengl, FilterMode, Filter, WrapMode, Wrap, pixel};
use crate::error::{GameError, GameResult};
use crate::math::{Size, Region};
use crate::engine::Engine;
use glow::Context;
use std::rc::Rc;
use std::path::Path;

pub struct Texture {
    texture: Rc<opengl::Texture>,
    size: Size<u32>,
    filter: Filter,
    mipmap_generated: bool,
    wrap: Wrap,
}

impl Texture {

    pub fn new<S: Into<Size<u32>>>(engine: &mut Engine, size: S, pixels: Option<&[u8]>) -> GameResult<Self> {
        let size = size.into();
        if let Some(pixels) = pixels {
            pixel::validate_pixels_len(size, pixels)?;
        }
        let filter = engine.graphics().default_filter();
        let generate_mipmap = filter.mipmap.is_some();
        let wrap = engine.graphics().default_wrap();
        let texture = opengl::Texture::new(engine.graphics().gl().clone())
            .map_err(|error| GameError::InitError(error.into()))?;
        texture.bind();
        texture.init_image(size.width, size.height, pixels);
        texture.set_filter(filter);
        if generate_mipmap {
            texture.generate_mipmap();
        }
        texture.set_wrap(wrap);
        texture.unbind();
        Ok(Self {
            texture: Rc::new(texture),
            size,
            filter,
            mipmap_generated: generate_mipmap,
            wrap,
        })
    }

    pub fn from_bytes(engine: &mut Engine, bytes: &[u8]) -> GameResult<Self> {
        let image = image::load_from_memory(bytes)
            .map_err(|error| GameError::InitError(Box::new(error)))?
            .into_rgba();
        let size = Size::new(image.width(), image.height());
        Self::new(engine, size, Some(image.into_raw().as_slice()))
    }

    pub fn load<P: AsRef<Path>>(engine: &mut Engine, path: P) -> GameResult<Self> {
        let bytes = engine.filesystem().read(path)?;
        Self::from_bytes(engine, &bytes)
    }

    pub(crate) fn white_1_x_1(gl: Rc<Context>) -> GameResult<Self> {
        let size = Size::new(1, 1);
        let pixels = [255, 255, 255, 255];
        pixel::validate_pixels_len(size, &pixels)?;
        let filter = Filter::new(FilterMode::Nearest, FilterMode::Nearest, None);
        let generate_mipmap = filter.mipmap.is_some();
        let wrap = Wrap::uv(WrapMode::Repeat, WrapMode::Repeat);
        let texture = opengl::Texture::new(gl)
            .map_err(|error| GameError::InitError(error.into()))?;
        texture.bind();
        texture.init_image(size.width, size.height, Some(&pixels));
        texture.set_filter(filter);
        if generate_mipmap {
            texture.generate_mipmap();
        }
        texture.set_wrap(wrap);
        texture.unbind();
        Ok(Self {
            texture: Rc::new(texture),
            size,
            filter,
            mipmap_generated: generate_mipmap,
            wrap,
        })
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    pub fn filter(&self) -> Filter {
        self.filter
    }

    pub fn set_filter(&mut self, filter: Filter) {
        if self.filter != filter {
            self.texture.bind();
            self.texture.set_filter(filter);
            if !self.mipmap_generated && filter.mipmap.is_some() {
                self.texture.generate_mipmap();
                self.mipmap_generated = true;
            }
            self.texture.unbind();
            self.filter = filter;
        }
    }

    pub fn wrap(&self) -> Wrap {
        self.wrap
    }

    pub fn set_wrap(&mut self, wrap: Wrap) {
        if self.wrap != wrap {
            self.texture.bind();
            self.texture.set_wrap(wrap);
            self.texture.unbind();
            self.wrap = wrap;
        }
    }

    pub fn init_pixels<S: Into<Size<u32>>>(&mut self, size: S, pixels: Option<&[u8]>) -> GameResult {
        let size = size.into();
        if let Some(pixels) = pixels {
            pixel::validate_pixels_len(size, pixels)?;
        }
        self.texture.bind();
        self.texture.init_image(size.width, size.height, pixels);
        self.size = size;
        if self.filter.mipmap.is_some() {
            self.texture.generate_mipmap();
            self.mipmap_generated = true;
        } else {
            self.mipmap_generated = false;
        }
        self.texture.unbind();
        Ok(())
    }

    pub fn update_pixels<R: Into<Region<u32>>>(&mut self, region: R, pixels: Option<&[u8]>) -> GameResult {
        let region = region.into();
        if let Some(pixels) = pixels {
            pixel::validate_pixels_len(region.size(), pixels)?;
        }
        self.texture.bind();
        self.texture.sub_image(
            region.x,
            region.y,
            region.width,
            region.height,
            pixels,
        );
        if self.filter.mipmap.is_some() {
            self.texture.generate_mipmap();
            self.mipmap_generated = true;
        } else {
            self.mipmap_generated = false;
        }
        self.texture.unbind();
        Ok(())
    }

}

pub(crate) trait TextureHolder {

    fn texture(&self) -> &Rc<opengl::Texture>;

}

impl TextureHolder for Texture {

    fn texture(&self) -> &Rc<opengl::Texture> {
        &self.texture
    }

}
