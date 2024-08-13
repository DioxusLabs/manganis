use manganis::*;

fn main() {
    // Generate a unqiue file name for the txt file after it's been bundled
    let txt_file: Asset = asset!("/assets/file.txt");
    println!("{txt_file}");

    // Same thing for images
    let image_file: Asset = asset!("/assets/logo.png");
    println!("{image_file}");

    // Customize the asset type by specifying the `image()` method
    let image_file: ImageAsset = asset!("/assets/logo.png".image());
    println!("{image_file}");

    // Images can be built with options
    let image_file: ImageAsset = asset!("/assets/logo.png"
        .image()
        .size(512, 512)
        .format(ImageType::Avif)
        .preload()
        .url_encoded()
        .low_quality_preview());

    // Folders too
    let folder_asset: FolderAsset = asset!("/assets/somefolder".folder());

    // css
    let css_asset: CssAsset = asset!("/assets/style.css".css().minify(true));

    // Json
    let json_asset: JsonAsset = asset!("/assets/data.json".json().preload().url_encoded());

    // Js
    let js_asset: JavaScriptAsset = asset!("/assets/script.js".javascript());

    // Ts
    let ts_asset: TypeScriptAsset = asset!("/assets/script.ts".typescript());

    // Video
    let video_asset: VideoAsset = asset!("/assets/video.mp4".video());
}

#[test]
fn parses_ur() {
    use fluent_uri::UriRef;
    type Uri = UriRef<String>;

    let uri = "data:,Hello%2C%20World%21";

    // "bundlename.bundle/my-image.png"
    // let uri = "bundle://pkg-name.bundle/my-image.png";
    // let uri = "bundle://pkg-name.bundle/my-image.png";
    // let uri = "bundle://bundlename.bundle/my-image.png";
    // let uri = "http://localhost:8080/assets/file.txt";
    // let uri = "bundle://app/assets/file.txt";

    let uri: Uri = uri.parse().unwrap();
    dbg!(&uri);
    dbg!(&uri);
}
