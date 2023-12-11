#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use manganis_macro::*;

/// An image asset
#[derive(Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct ImageAsset {
    /// The path to the image
    path: &'static str,
    /// A low quality preview of the image that is URL encoded
    preview: Option<&'static str>,
    /// A caption for the image
    caption: Option<&'static str>,
}

impl ImageAsset {
    /// Creates a new image asset
    pub const fn new(path: &'static str) -> Self {
        Self {
            path,
            preview: None,
            caption: None,
        }
    }

    /// Returns the path to the image
    pub const fn path(&self) -> &'static str {
        self.path
    }

    /// Returns the preview of the image
    pub const fn preview(&self) -> Option<&'static str> {
        self.preview
    }

    /// Sets the preview of the image
    pub const fn with_preview(self, preview: Option<&'static str>) -> Self {
        Self { preview, ..self }
    }

    /// Returns the caption of the image
    pub const fn caption(&self) -> Option<&'static str> {
        self.caption
    }

    /// Sets the caption of the image
    pub const fn with_caption(self, caption: Option<&'static str>) -> Self {
        Self { caption, ..self }
    }
}

impl std::ops::Deref for ImageAsset {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.path
    }
}

impl std::fmt::Display for ImageAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path.fmt(f)
    }
}

#[cfg(feature = "dioxus")]
impl<'a> dioxus_core::prelude::IntoAttributeValue<'a> for ImageAsset {
    fn into_value(
        self,
        _: &'a dioxus_core::exports::bumpalo::Bump,
    ) -> dioxus_core::AttributeValue<'a> {
        dioxus_core::AttributeValue::Text(self.path)
    }
}

/// A builder for an image asset. This must be used in the `mg!` macro.
pub struct ImageAssetBuilder;

impl ImageAssetBuilder {
    /// Sets the preview of the image
    #[allow(unused)]
    pub const fn format(self, format: manganis_common::ImageType) -> Self {
        Self
    }

    /// Sets the size of the image
    #[allow(unused)]
    pub const fn size(self, size: Option<manganis_common::ImageType>) -> Self {
        Self
    }

    /// Make the image use a low quality preview
    #[allow(unused)]
    pub const fn low_quality_preview(self) -> Self {
        Self
    }

    /// Make the image preloaded
    #[allow(unused)]
    pub const fn preload(self) -> Self {
        Self
    }

    /// Make the image URL encoded
    #[allow(unused)]
    pub const fn url_encoded(self) -> Self {
        Self
    }
}

/// Create an image asset from the local path to the image
#[allow(unused)]
const fn image(path: &'static str) -> ImageAssetBuilder {
    ImageAssetBuilder
}

/// A builder for a font asset. This must be used in the `mg!` macro.
pub struct FontAssetBuilder;

impl FontAssetBuilder {
    /// Sets the font family of the font
    #[allow(unused)]
    pub const fn family(self, family: &'static str) -> Self {
        Self
    }

    /// Sets the font weight of the font
    #[allow(unused)]
    pub const fn weights<const N: usize>(self, weights: [u32; N]) -> Self {
        Self
    }

    /// Sets the subset of text that the font needs to support
    #[allow(unused)]
    pub const fn text(self, text: &'static str) -> Self {
        Self
    }

    /// Sets the display of the font
    #[allow(unused)]
    pub const fn display(self, display: &'static str) -> Self {
        Self
    }
}

/// Create a font asset from the local path to the font
#[allow(unused)]
const fn font(path: &'static str) -> FontAssetBuilder {
    FontAssetBuilder
}

/// A trait for something that can be used in the `mg!` macro
pub trait ForMgMacro {}

impl ForMgMacro for ImageAssetBuilder {}
impl ForMgMacro for FontAssetBuilder {}
impl ForMgMacro for &'static str {}
