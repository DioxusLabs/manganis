# Dioxus Assets

Dioxus Assets handles collecting assets through dependencies and compressing images, videos, fonts, CSS, and tailwind assets.

If you defined this in a component library:
```rust
const AVIF_ASSET: &str = assets::file!("./rustacean-flat-gesture.png" -> avif);
```

AVIF_ASSET will be set to a new file name that will be served by some CLI with the avif encoding. That file can be collected by any package that depends on the component library.

TODO:
- [ ] Support optimizing assets
- - [x] PNG
- - [x] JPG
- - [x] Convert images
- - [x] Resize images
- - [ ] GIF
- - [ ] Convert videos
- - [ ] Convert fonts
- - [ ] CSS
- [x] Collect assets from dependencies
- [ ] Tailwind
- [ ] Think of a better name
- [ ] Configuration for the final asset serve location
