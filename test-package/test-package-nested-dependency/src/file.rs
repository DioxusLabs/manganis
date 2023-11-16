const _: &str = manganis::classes!("flex flex-row p-5");
pub const CSS_ASSET: &str = manganis::file!("./style.css");
pub const PNG_ASSET: &str = manganis::file!("./rustacean-flat-gesture.png");
pub const RESIZED_PNG_ASSET: manganis::ImageAsset =
    manganis::image!("./rustacean-flat-gesture.png", { size: (52, 52) });
pub const JPEG_ASSET: manganis::ImageAsset =
    manganis::image!("./rustacean-flat-gesture.png", { format: jpeg });
pub const RESIZED_JPEG_ASSET: manganis::ImageAsset =
    manganis::image!("./rustacean-flat-gesture.png", { format: jpeg, size: (52, 52) });
pub const AVIF_ASSET: manganis::ImageAsset =
    manganis::image!("./rustacean-flat-gesture.png", { format: avif, low_quality_preview: true });
pub const RESIZED_AVIF_ASSET: manganis::ImageAsset =
    manganis::image!("./rustacean-flat-gesture.png", { format: avif, size: (52, 52) });
pub const WEBP_ASSET: manganis::ImageAsset =
    manganis::image!("./rustacean-flat-gesture.png", { format: webp });
pub const RESIZED_WEBP_ASSET: manganis::ImageAsset =
    manganis::image!("./rustacean-flat-gesture.png", { format: webp, size: (52, 52) });
