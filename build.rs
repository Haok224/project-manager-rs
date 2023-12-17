use chrono::Local;
use slint_build::CompilerConfiguration;

fn main() {
    // Slint Build
    slint_build::compile_with_config(
        "ui/ui.slint",
        CompilerConfiguration::new().with_style("fluent-light".to_string()),
    )
    .unwrap();

    // 编译时间
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("cargo:rustc-env=BUILD_TIME={}", timestamp);
}
