fn main() {
    if cfg!(feature = "boot_from_flash") {
        println!("cargo:rustc-link-arg=-Trom.ld");
    } else {
        println!("cargo:rustc-link-arg=-Tram.ld");
    }
}
