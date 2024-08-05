#![doc = include_str!("../../../README.md")]
#![deny(missing_docs)]

#[cfg(feature = "macro")]
pub use manganis_macro::*;

/// An image asset that is built by the [`mg!`] macro
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

/// The type of an image. You can read more about the tradeoffs between image formats [here](https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Image_types)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub enum ImageType {
    /// A png image. Png images cannot contain transparency and tend to compress worse than other formats
    Png,
    /// A jpg image. Jpg images can contain transparency and tend to compress better than png images
    Jpg,
    /// A webp image. Webp images can contain transparency and tend to compress better than jpg images
    Webp,
    /// An avif image. Avif images can compress slightly better than webp images but are not supported by all browsers
    Avif,
}

/// A builder for an image asset. This must be used in the [`mg!`] macro.
///
/// > **Note**: This will do nothing outside of the `mg!` macro
pub struct ImageAssetBuilder;

impl ImageAssetBuilder {
    /// Sets the format of the image
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// Choosing the right format can make your site load much faster. Webp and avif images tend to be a good default for most images
    ///
    /// ```rust
    /// const _: manganis::ImageAsset = manganis::asset!(image("https://avatars.githubusercontent.com/u/79236386?s=48&v=4").format(ImageType::Webp));
    /// ```
    #[allow(unused)]
    pub const fn format(self, format: ImageType) -> Self {
        Self
    }

    /// Sets the size of the image
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// If you only use the image in one place, you can set the size of the image to the size it will be displayed at. This will make the image load faster
    ///
    /// ```rust
    /// const _: manganis::ImageAsset = manganis::asset!(image("https://avatars.githubusercontent.com/u/79236386?s=48&v=4").size(512, 512));
    /// ```
    #[allow(unused)]
    pub const fn size(self, x: u32, y: u32) -> Self {
        Self
    }

    /// Make the image use a low quality preview
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// A low quality preview is a small version of the image that will load faster. This is useful for large images on mobile devices that may take longer to load
    ///
    /// ```rust
    /// const _: manganis::ImageAsset = manganis::asset!(image("https://avatars.githubusercontent.com/u/79236386?s=48&v=4").low_quality_preview());
    /// ```
    #[allow(unused)]
    pub const fn low_quality_preview(self) -> Self {
        Self
    }

    /// Make the image preloaded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// Preloading an image will make the image start to load as soon as possible. This is useful for images that will be displayed soon after the page loads or images that may not be visible immediately, but should start loading sooner
    ///
    /// ```rust
    /// const _: manganis::ImageAsset = manganis::asset!(image("https://avatars.githubusercontent.com/u/79236386?s=48&v=4").preload());
    /// ```
    #[allow(unused)]
    pub const fn preload(self) -> Self {
        Self
    }

    /// Make the image URL encoded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// URL encoding an image inlines the data of the image into the URL. This is useful for small images that should load as soon as the html is parsed
    ///
    /// ```rust
    /// const _: manganis::ImageAsset = manganis::asset!(image("https://avatars.githubusercontent.com/u/79236386?s=48&v=4").url_encoded());
    /// ```
    #[allow(unused)]
    pub const fn url_encoded(self) -> Self {
        Self
    }
}

/// Create an image asset from the local path or url to the image
///
/// > **Note**: This will do nothing outside of the `mg!` macro
///
/// You can collect images which will be automatically optimized with the image builder:
/// ```rust
/// const _: manganis::ImageAsset = manganis::asset!(image("rustacean-flat-gesture.png"));
/// ```
/// Resize the image at compile time to make the assets file size smaller:
/// ```rust
/// const _: manganis::ImageAsset = manganis::asset!(image("rustacean-flat-gesture.png").size(52, 52));
/// ```
/// Or convert the image at compile time to a web friendly format:
/// ```rust
/// const _: manganis::ImageAsset = manganis::asset!(image("rustacean-flat-gesture.png").format(ImageType::Avif).size(52, 52));
/// ```
/// You can mark images as preloaded to make them load faster in your app
/// ```rust
/// const _: manganis::ImageAsset = manganis::asset!(image("rustacean-flat-gesture.png").preload());
/// ```
#[allow(unused)]
pub const fn image(path: &'static str) -> ImageAssetBuilder {
    ImageAssetBuilder
}

/// A builder for a css asset. This must be used in the [`mg!`] macro.
///
/// > **Note**: This will do nothing outside of the `mg!` macro
pub struct CssAssetBuilder;

impl CssAssetBuilder {
    /// Sets whether the css should be minified (default: true)
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// Minifying the css can make your site load faster by loading less data
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(css("https://sindresorhus.com/github-markdown-css/github-markdown.css").minify(false));
    /// ```
    #[allow(unused)]
    pub const fn minify(self, minify: bool) -> Self {
        Self
    }

    /// Make the css preloaded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// Preloading css will make the css start to load as soon as possible. This is useful for css that will be displayed soon after the page loads or css that may not be visible immediately, but should start loading sooner
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(css("https://sindresorhus.com/github-markdown-css/github-markdown.css").preload());
    /// ```
    #[allow(unused)]
    pub const fn preload(self) -> Self {
        Self
    }

    /// Make the css URL encoded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// URL encoding an image inlines the data of the css into the URL. This is useful for small css files that should load as soon as the html is parsed
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(css("https://sindresorhus.com/github-markdown-css/github-markdown.css").url_encoded());
    /// ```
    #[allow(unused)]
    pub const fn url_encoded(self) -> Self {
        Self
    }
}

/// A builder for a javascript asset. This must be used in the [`mg!`] macro.
///
/// > **Note**: This will do nothing outside of the `mg!` macro
pub struct JsAssetBuilder;

impl JsAssetBuilder {
    /// Sets whether the js should be minified (default: true)
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// Minifying the js can make your site load faster by loading less data
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(js("assets/script.js").minify(false));
    /// ```
    #[allow(unused)]
    pub const fn minify(self, minify: bool) -> Self {
        Self
    }

    /// Make the js preloaded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// Preloading js will make the js start to load as soon as possible. This is useful for js that will be run soon after the page loads or js that may not be used immediately, but should start loading sooner
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(js("assets/script.js").preload());
    /// ```
    #[allow(unused)]
    pub const fn preload(self) -> Self {
        Self
    }

    /// Make the js URL encoded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// URL encoding an image inlines the data of the js into the URL. This is useful for small js files that should load as soon as the html is parsed
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(js("assets/script.js").url_encoded());
    /// ```
    #[allow(unused)]
    pub const fn url_encoded(self) -> Self {
        Self
    }
}

/// A builder for a json asset. This must be used in the [`mg!`] macro.
///
/// > **Note**: This will do nothing outside of the `mg!` macro
pub struct JsonAssetBuilder;

impl JsonAssetBuilder {
    /// Make the json preloaded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// Preloading json will make the json start to load as soon as possible. This is useful for json that will be run soon after the page loads or json that may not be used immediately, but should start loading sooner
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(json("assets/data.json").preload());
    /// ```
    #[allow(unused)]
    pub const fn preload(self) -> Self {
        Self
    }

    /// Make the json URL encoded
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// URL encoding an image inlines the data of the json into the URL. This is useful for small json files that should load as soon as the html is parsed
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(json("assets/data.json").url_encoded());
    /// ```
    #[allow(unused)]
    pub const fn url_encoded(self) -> Self {
        Self
    }
}

/// Create an css asset from the local path or url to the css
///
/// > **Note**: This will do nothing outside of the `mg!` macro
///
/// You can collect css which will be automatically minified with the css builder:
/// ```rust
/// const _: &str = manganis::asset!(css("https://sindresorhus.com/github-markdown-css/github-markdown.css"));
/// ```
/// You can mark css as preloaded to make them load faster in your app:
/// ```rust
/// const _: &str = manganis::asset!(css("https://sindresorhus.com/github-markdown-css/github-markdown.css").preload());
/// ```
#[allow(unused)]
pub const fn css(path: &'static str) -> CssAssetBuilder {
    CssAssetBuilder
}

/// A builder for a font asset. This must be used in the `mg!` macro.
///
/// > **Note**: This will do nothing outside of the `mg!` macro
pub struct FontAssetBuilder;

impl FontAssetBuilder {
    /// Sets the font family of the font
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(font().families(["Roboto"]));
    /// ```
    #[allow(unused)]
    pub const fn families<const N: usize>(self, families: [&'static str; N]) -> Self {
        Self
    }

    /// Sets the font weight of the font
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(font().families(["Roboto"]).weights([200]));
    /// ```
    #[allow(unused)]
    pub const fn weights<const N: usize>(self, weights: [u32; N]) -> Self {
        Self
    }

    /// Sets the subset of text that the font needs to support. The font will only include the characters in the text which can make the font file size significantly smaller
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(font().families(["Roboto"]).weights([200]).text("Hello, world!"));
    /// ```
    #[allow(unused)]
    pub const fn text(self, text: &'static str) -> Self {
        Self
    }

    /// Sets the [display](https://www.w3.org/TR/css-fonts-4/#font-display-desc) of the font. The display control what happens when the font is unavailable
    ///
    /// > **Note**: This will do nothing outside of the `mg!` macro
    ///
    /// ```rust
    /// const _: &str = manganis::asset!(font().families(["Roboto"]).weights([200]).text("Hello, world!").display("swap"));
    /// ```
    #[allow(unused)]
    pub const fn display(self, display: &'static str) -> Self {
        Self
    }
}

/// Create a font asset
///
/// > **Note**: This will do nothing outside of the `mg!` macro
///
/// You can use the font builder to collect fonts that will be included in the final binary from google fonts
/// ```rust
/// const _: &str = manganis::asset!(font().families(["Roboto"]));
/// ```
/// You can specify weights for the fonts
/// ```rust
/// const _: &str = manganis::asset!(font().families(["Roboto"]).weights([200]));
/// ```
/// Or set the text to only include the characters you need
/// ```rust
/// const _: &str = manganis::asset!(font().families(["Roboto"]).weights([200]).text("Hello, world!"));
/// ```
#[allow(unused)]
pub const fn font() -> FontAssetBuilder {
    FontAssetBuilder
}

/// Create an file asset from the local path or url to the file
///
/// > **Note**: This will do nothing outside of the `mg!` macro
///
/// The file builder collects an arbitrary file. Relative paths are resolved relative to the package root
/// ```rust
/// const _: &str = manganis::asset!("/assets/asset.txt");
/// ```
/// Or you can use URLs to read the asset at build time from a remote location
/// ```rust
/// const _: &str = manganis::asset!("https://rustacean.net/assets/rustacean-flat-happy.png");
/// ```
#[allow(unused)]
pub const fn file(path: &'static str) -> &'static str {
    path
}

/// Create an folder asset from the local path
///
/// > **Note**: This will do nothing outside of the `mg!` macro
///
/// The folder builder collects an arbitrary local folder. Relative paths are resolved relative to the package root
/// ```rust
/// const _: &str = manganis::asset!("/assets");
/// ```
#[allow(unused)]
pub const fn folder(path: &'static str) -> &'static str {
    path
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
    impl Sealed for JsAssetBuilder {}
    impl Sealed for JsonAssetBuilder {}
    impl Sealed for CssAssetBuilder {}
    impl Sealed for &'static str {}
}

impl ForMgMacro for ImageAssetBuilder {}
impl ForMgMacro for FontAssetBuilder {}
impl ForMgMacro for &'static str {}
