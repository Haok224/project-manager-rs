use std::{
    ffi::CString,
    io::{Read, Write},
};

use toml::{Table, Value};
use winapi::{shared::windef::HWND__, um::winuser::MB_ICONERROR};

use super::super::show_error_with_args;

pub enum Actions {
    DefaultProjectPath,
    NowTab,
}

pub fn set_property(action: Actions, path: &str, arg: &str, _hwnd: *mut HWND__) {
    let mut config = read_property(_hwnd, path);
    println!("{path} {arg}");
    let mut ss = String::new();
    let mut file = {
        let fff = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap_or_else(show_error_with_args!(
                std::io::Error,
                "无法读取位于 {1} 的配置文件.\n原因:\n{0}",
                path
            ));
        fff
    };
    file.read_to_string(&mut ss)
        .unwrap_or_else(show_error_with_args!(
            std::io::Error,
            "无法读取位于 {1} 的配置文件.\n原因:\n{0}",
            path
    ));
    println!("READ CONFIG = {config}");
    match action {
        Actions::DefaultProjectPath => {
            config.insert(
                "default-project-path".into(),
                Value::String(arg.to_string()),
            );
            println!("{}", config.to_string());
        }
        Actions::NowTab => {
            config.insert("nowtab".into(), Value::String(format!("{arg}")));
        }
    }
    println!("{}", config.to_string());
    file.write_all(config.to_string().as_bytes())
        .unwrap_or_else(show_error_with_args!(
            std::io::Error,
            "无法写入位于{1}的配置文件.\n原因:\n{0}",
            path
        ));
}

pub fn read_property(_hwnd: *mut HWND__, path: &str) -> Table {
    println!("{path}");
    let mut ss = String::new();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap_or_else(show_error_with_args!(
            std::io::Error,
            "无法读取位于{1}的配置文件.\n原因:\n{0}",
            path
        ));
    file.read_to_string(&mut ss)
        .unwrap_or_else(show_error_with_args!(
            std::io::Error,
            "无法读取位于{1}的配置文件.\n原因:\n{0}",
            path
        ));
    let config = ss.parse::<Table>().unwrap_or_else(show_error_with_args!(
        toml::de::Error,
        "无法解析位于 {1} 的配置文件.\n原因:\n{0}",
        path
    ));
    println!("{}", config.to_string());
    config
}
