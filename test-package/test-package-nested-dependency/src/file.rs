const _: &str = collect_assets::classes!("flex flex-row p-5");
pub const CSS_ASSET: &str = collect_assets::file!("./style.css");
pub const PNG_ASSET: &str = collect_assets::file!("./rustacean-flat-gesture.png");
pub const RESIZED_PNG_ASSET: &str =
    collect_assets::image!("./rustacean-flat-gesture.png", { size: (52, 52) });
pub const JPEG_ASSET: &str =
    collect_assets::image!("./rustacean-flat-gesture.png", { format: jpeg });
pub const RESIZED_JPEG_ASSET: &str =
    collect_assets::image!("./rustacean-flat-gesture.png", { format: jpeg, size: (52, 52) });
pub const AVIF_ASSET: &str =
    collect_assets::image!("./rustacean-flat-gesture.png", { format: avif });
pub const RESIZED_AVIF_ASSET: &str =
    collect_assets::image!("./rustacean-flat-gesture.png", { format: avif, size: (52, 52) });
pub const WEBP_ASSET: &str =
    collect_assets::image!("./rustacean-flat-gesture.png", { format: webp });
pub const RESIZED_WEBP_ASSET: &str =
    collect_assets::image!("./rustacean-flat-gesture.png", { format: webp, size: (52, 52) });
