#[cfg(all(target_os = "windows"))]
fn main() {
    use winres::WindowsResource;

    let mut res = WindowsResource::new();
    res.set_manifest_file("./app.manifest");
    if let Err(error) = res.compile() {
        eprint!("{error}");
        std::process::exit(1);
    }
}
