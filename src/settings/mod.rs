mod editor;

pub mod config {
    use winapi::shared::windef::HWND__;

    use super::editor::{self, Actions};

    pub fn set_default_path(path: &str, arg: &str, hwnd: *mut HWND__) {
        editor::set_property(Actions::DefaultProjectPath, arg, path, hwnd);
    }
}
