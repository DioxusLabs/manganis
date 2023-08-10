use assets::asset;

fn main() {
    asset!("Hello, world!");
    let assets = assets::all_assets();
    println!("{:?}", assets);
}
