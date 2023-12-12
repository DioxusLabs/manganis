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

/// The type of an image
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub enum ImageType {
    /// A png image
    Png,
    /// A jpg image
    Jpg,
    /// An avif image
    Avif,
    /// A webp image
    Webp,
}

/// A builder for an image asset. This must be used in the `mg!` macro.
///
/// > **Note**: This will do nothing outside of the `mg!` macro
pub struct ImageAssetBuilder;

impl ImageAssetBuilder {
    /// Sets the preview of the image
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn format(self, format: ImageType) -> Self {
        Self
    }

    /// Sets the size of the image
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn size(self, x: u32, y: u32) -> Self {
        Self
    }

    /// Make the image use a low quality preview
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn low_quality_preview(self) -> Self {
        Self
    }

    /// Make the image preloaded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn preload(self) -> Self {
        Self
    }

    /// Make the image URL encoded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn url_encoded(self) -> Self {
        Self
    }
}

/// Create an image asset from the local path to the image
///
/// > **Note**: This will do nothing outside of the `mg!` macro
#[allow(unused)]
pub const fn image(path: &'static str) -> ImageAssetBuilder {
    ImageAssetBuilder
}

/// A builder for a font asset. This must be used in the `mg!` macro.
///
/// > **Note**: This will do nothing outside of the `mg!` macro
pub struct FontAssetBuilder;

impl FontAssetBuilder {
    /// Sets the font family of the font
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn families<const N: usize>(self, families: [&'static str; N]) -> Self {
        Self
    }

    /// Sets the font weight of the font
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn weights<const N: usize>(self, weights: [u32; N]) -> Self {
        Self
    }

    /// Sets the subset of text that the font needs to support
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn text(self, text: &'static str) -> Self {
        Self
    }

    /// Sets the display of the font
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    #[allow(unused)]
    pub const fn display(self, display: &'static str) -> Self {
        Self
    }
}

/// Create a font asset
///
/// > **Note**: This will do nothing outside of the `mg!` macro
#[allow(unused)]
pub const fn font() -> FontAssetBuilder {
    FontAssetBuilder
}

/// Create an file asset from the local path or url to the file
///
/// > **Note**: This will do nothing outside of the `mg!` macro
#[allow(unused)]
pub const fn file(path: &'static str) -> ImageAssetBuilder {
    ImageAssetBuilder
}

/// A trait for something that can be used in the `mg!` macro
///
/// > **Note**: These types will do nothing outside of the `mg!` macro
pub trait ForMgMacro: __private::Sealed + Sync + Send {}

mod __private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for ImageAssetBuilder {}
    impl Sealed for FontAssetBuilder {}
    impl Sealed for &'static str {}
}

impl ForMgMacro for ImageAssetBuilder {}
impl ForMgMacro for FontAssetBuilder {}
impl ForMgMacro for &'static str {}
