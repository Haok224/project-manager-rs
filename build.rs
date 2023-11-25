fn main() {
    slint_build::compile("ui/ui.slint").unwrap();
    cc::Build::new().file("csrc/icon.c").compile("icon");
    println!("cargo:rerun-if-changed=csrc/icon.c");
}
