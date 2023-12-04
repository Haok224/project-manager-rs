mod editor;

pub mod config {
    use std::ffi::CString;
    use winapi::{shared::windef::HWND__, um::winuser::MB_ICONERROR};

    use super::editor::{self, Actions};

    pub fn set_default_path(path: &str, arg: &str, hwnd: *mut HWND__) {
        editor::set_property(Actions::DefaultProjectPath, arg, path, hwnd);
    }
    pub fn read_default_path(hwnd: *mut HWND__, path: &str) -> String {
        let config = editor::read_property(hwnd, path);
        config
            .get("default-project-path")
            .unwrap_or_else(|| -> _ {
                unsafe {
                    let message = CString::new("ERROR").unwrap();
                    let title = CString::new(format!("无法读取位于{path}的配置文件.")).unwrap();
                    winapi::um::winuser::MessageBoxA(
                        hwnd,
                        title.as_ptr(),
                        message.as_ptr(),
                        MB_ICONERROR,
                    )
                };
                panic!("Cannot open config file.");
            })
            .as_str()
            .unwrap_or_else(|| -> _ {
                unsafe {
                    let message = CString::new("ERROR").unwrap();
                    let title = CString::new(format!("无法读取位于{path}的配置文件.")).unwrap();
                    winapi::um::winuser::MessageBoxA(
                        hwnd,
                        title.as_ptr(),
                        message.as_ptr(),
                        MB_ICONERROR,
                    )
                };
                panic!("Cannot open config file.");
            })
            .to_string()
    }
}
