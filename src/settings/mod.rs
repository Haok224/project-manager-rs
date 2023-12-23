pub mod editor;

pub mod config {
    use std::ffi::CString;
    use winapi::{shared::windef::HWND__, um::winuser::MB_ICONERROR};

    use crate::show_error;

    use super::editor::{self, Actions};

    pub fn set_default_path(path: &str, arg: &str, hwnd: *mut HWND__) {
        editor::set_property(Actions::DefaultProjectPath, arg, path, hwnd);
    }
    pub fn set_nowtab(path: &str, arg: &str, hwnd: *mut HWND__) {
        editor::set_property(Actions::NowTab, path, arg, hwnd);
    }
    pub fn read_default_path(hwnd: *mut HWND__, path: &str) -> String {
        let config = editor::read_property(hwnd, path);
        config
            .get("default-project-path")
            .unwrap_or_else(show_error!("无法读取位于{0}的配置文件.", path))
            .as_str()
            .unwrap_or_else(show_error!("无法读取位于{0}的配置文件.", path))
            .to_string()
    }
    pub fn read_nowtab(_hwnd: *mut HWND__, path: &str) -> Option<String> {
        let config = editor::read_property(_hwnd, path);
        Some(config.get("nowtab")?.as_str()?.into())
    }
}
