use std::ptr;
use std::ffi::CString;


use winapi::um::winuser::{FindWindowA, SendMessageA, WM_SETICON, IMAGE_ICON};

pub fn set_window_icon(title: &str, path: &str) ->bool{
    unsafe {
        let title_cstring = CString::new(title).expect("Failed to convert title to CString");
        let title_ptr = title_cstring.as_ptr();

        let path_cstring = CString::new(path).expect("Failed to convert path to CString");

        let hwnd = FindWindowA(ptr::null(), title_ptr);
        if hwnd.is_null() {
            println!("Window with title '{}' not found", title);
            return false;
        }

        let path_ptr = path_cstring.as_ptr();
        let result = SendMessageA(hwnd, WM_SETICON, IMAGE_ICON.try_into().unwrap(), path_ptr as isize);
        result>0
    }
}