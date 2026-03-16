fn main() {
    // delete existing version file created by blink.lib.build.download
    let _ = std::fs::remove_file("target/release/version");
}
