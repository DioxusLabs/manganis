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
