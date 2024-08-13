use manganis::*;

fn main() {
    // Generate a unqiue file name for the txt file after it's been bundled
    let txt_file: Asset = asset!("/assets/file.txt");
    println!("{txt_file}");

    // Generate a unqiue file name for the txt file after it's been bundled
    let image_file: Asset = asset!("/assets/logo.png");
    println!("{image_file}");

    // Urls too
    let url = asset!("https://raw.githubusercontent.com/TheZoq2/ferris/2c58ca0909375fcf8a21ce0296fb320e5bdf5bea/book_cover/space.png");
    println!("{url}");

    // Images
    let image_file: Asset = asset!("/assets/logo.png");
    println!("{image_file}");

    // Folders too
    let folder_asset: Asset = asset!("/assets/somefolder");
    let folder_asset: Asset = asset!(folder("/assets/somefolder"));

    // // todo: this doesn't work yet
    // // Relative assets
    // let file = asset!("relative.txt");
    // print!("{file}");
}

#[test]
fn parses_ur() {
    use fluent_uri::UriRef;
    type Uri = UriRef<String>;

    // "bundlename.bundle/my-image.png"
    let uri = "data:,Hello%2C%20World%21";
    // let uri = "bundle://pkg-name.bundle/my-image.png";
    // let uri = "bundle://pkg-name.bundle/my-image.png";
    // let uri = "bundle://bundlename.bundle/my-image.png";

    // let uri = "http://localhost:8080/assets/file.txt";
    // let uri = "bundle://app/assets/file.txt";
    let uri: Uri = uri.parse().unwrap();
    dbg!(&uri);
    dbg!(&uri);
}
