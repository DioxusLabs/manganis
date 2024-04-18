pub static FORCE_IMPORT : u32 = 0;

const _: &str = manganis::classes!("flex flex-col p-5");
pub const TEXT_ASSET: &str = manganis::mg!(file("./src/asset.txt"));
pub const IMAGE_ASSET: &str = manganis::mg!(file(
    "https://rustacean.net/assets/rustacean-flat-happy.png"
));
pub const HTML_ASSET: &str = manganis::mg!(file("https://github.com/DioxusLabs/dioxus"));
