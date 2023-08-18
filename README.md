# Manganis

The Manganis allows you to submit assets to a build tool that supports collecting assets. It makes it easy to self-host assets that are distributed throughout your library. Manganis also handles optimizing, converting, and fetching assets.

If you defined this in a component library:
```rust
const AVIF_ASSET: &str = manganis::file!("./rustacean-flat-gesture.png");
```

AVIF_ASSET will be set to a new file name that will be served by some CLI. That file can be collected by any package that depends on the component library.

```rust
// You can include tailwind classes that will be collected into the final binary
const TAILWIND_CLASSES: &str = manganis::classes!("flex flex-col p-5");

// You can also collect arbitrary files. Relative paths are resolved relative to the package root
const _: &str = manganis::file!("./src/asset.txt");
// You can use URLs to copy read the asset at build time
const _: &str = manganis::file!("https://rustacean.net/assets/rustacean-flat-happy.png");

// You can collect images which will be automatically optimized
const _: &str = manganis::image!("./rustacean-flat-gesture.png");
// Resize the image at compile time to make the assets smaller
const _: &str = manganis::image!("./rustacean-flat-gesture.png", { size: (52, 52) });
// Or convert the image at compile time to a web friendly format
const _: &str = manganis::image!("./rustacean-flat-gesture.png", { format: avif, size: (52, 52) });

// You can also collect google fonts
const _: &str = manganis::font!({ families: ["Roboto"] });
// Specify weights for fonts to collect
const _: &str = manganis::font!({ families: ["Comfortaa"], weights: [300] });
// Or specific text to include fonts for only the characters used in that text
const _: &str = manganis::font!({ families: ["Roboto"], weights: [200], text: "light font" });
```

## Adding Support to Your CLI

To add support for 

TODO:
- [x] Support optimizing assets
- - [x] PNG
- - [x] JPG
- - [x] Convert images
- - [x] Resize images
- - [x] CSS
- - [x] Self-host remote assets (fonts)
- - [ ] JS (?)
- [x] Google Fonts Integration
- [x] Deduplicate assets
- [x] Collect assets from dependencies
- [x] Tailwind
- [x] Configuration for the final asset serve location
- [x] `#![deny(missing_docs)]`
- [ ] Think of a better name
