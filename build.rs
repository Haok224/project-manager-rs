use slint_build::CompilerConfiguration;

fn main() {
    slint_build::compile_with_config(
        "ui/ui.slint",
        CompilerConfiguration::new().with_style("fluent-light".to_string()),
    )
    .unwrap();
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}

