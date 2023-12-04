use std::{
    ffi::CString,
    io::{Read, Write},
};

use toml::{Table, Value};
use winapi::{shared::windef::HWND__, um::winuser::MB_ICONERROR};

pub enum Actions {
    DefaultProjectPath,
}

pub fn set_property(action: Actions, path: &str, arg: &str, hwnd: *mut HWND__) {
    println!("{path} {arg}");
    let mut ss = String::new();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap_or_else(|e| -> _ {
            unsafe {
                let message = CString::new("ERROR").unwrap();
                let title =
                    CString::new(format!("无法读取位于 {path} 的配置文件.\n原因:\n{e}")).unwrap();
                winapi::um::winuser::MessageBoxA(
                    hwnd,
                    title.as_ptr(),
                    message.as_ptr(),
                    MB_ICONERROR,
                )
            };
            panic!("Cannot open config file.");
        });
    file.read_to_string(&mut ss).unwrap_or_else(|e| -> _ {
        unsafe {
            let message = CString::new("ERROR").unwrap();
            let title = CString::new(format!("无法读取位于{path}的配置文件.\n原因:\n{e}")).unwrap();
            winapi::um::winuser::MessageBoxA(hwnd, title.as_ptr(), message.as_ptr(), MB_ICONERROR)
        };
        panic!("Cannot open config file.");
    });
    let mut config = ss.parse::<Table>().unwrap_or_else(|e| -> _ {
        unsafe {
            let message = CString::new("ERROR").unwrap();
            let title = CString::new(format!("无法解析位于{path}的配置文件.\n原因:\n{e}")).unwrap();
            winapi::um::winuser::MessageBoxA(hwnd, message.as_ptr(), title.as_ptr(), MB_ICONERROR)
        };
        panic!("Cannot open config file.");
    });
    match action {
        Actions::DefaultProjectPath => {
            config.insert(
                "default-project-path".into(),
                Value::String(arg.to_string()),
            );
            println!("{}", config.to_string());
        }
    }
    println!("{}", config.to_string());
    //写入到配置文件
    file.write_all(config.to_string().as_bytes())
        .unwrap_or_else(|e| -> _ {
            unsafe {
                let message = CString::new("ERROR").unwrap();
                let title =
                    CString::new(format!("无法写入位于{path}的配置文件.\n原因:\n{e}")).unwrap();
                winapi::um::winuser::MessageBoxA(
                    hwnd,
                    title.as_ptr(),
                    message.as_ptr(),
                    MB_ICONERROR,
                )
            };
            panic!("Cannot open config file.");
        });
}
