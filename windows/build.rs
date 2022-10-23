fn main() {
    if cfg!(target = "x86_64-pc-windows-gnu") {
        println!("cargo:rustc-link-arg=-mwindows");
    }
    embed_resource::compile("manifest.rc");
}
