//! Sub-texture [Texture].
use crate::{rect::Rect, texture::Texture};
use std::borrow::Cow;

/// Texture that is a subset of another texture.
pub struct SubTexture<'a, T: 'a + Clone> {
    texture: Cow<'a, T>,
    source: Rect,
}

impl<'a, T: Texture + Clone> SubTexture<'a, T> {
    /// Get a `SubTexture` owning the parent texture.
    pub fn new(texture: T, source: Rect) -> SubTexture<'a, T> {
        SubTexture {
            texture: Cow::Owned(texture),
            source: source,
        }
    }

    /// Get a `SubTexture` referencing the parent texture.
    pub fn from_ref(texture: &'a T, source: Rect) -> SubTexture<'a, T> {
        SubTexture {
            texture: Cow::Borrowed(texture),
            source: source,
        }
    }
}

impl<'a, T: Texture + Clone> Texture for SubTexture<'a, T> {
    type Pixel = T::Pixel;

    fn width(&self) -> u32 {
        self.source.w
    }

    fn height(&self) -> u32 {
        self.source.h
    }

    fn get(&self, x: u32, y: u32) -> Option<T::Pixel> {
        let x = self.source.x + x;
        let y = self.source.y + y;
        self.texture.get(x, y)
    }

    fn set(&mut self, x: u32, y: u32, val: T::Pixel) {
        if let Cow::Owned(ref mut t) = self.texture {
            let x = self.source.x + x;
            let y = self.source.y + y;
            t.set(x, y, val);
        } else {
            panic!("Can't set pixel by borrowed reference");
        }
    }
}
