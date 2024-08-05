use manganis::*;

fn main() {
    // Generate a unqiue file name for the txt file after it's been bundled
    let txt_file = asset!("/assets/file.txt");
    println!("{txt_file}");

    // Generate a unqiue file name for the txt file after it's been bundled
    let image_file = asset!("/assets/logo.png");
    println!("{image_file}");
}
