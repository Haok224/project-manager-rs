use std::{
    ffi::CString,
    io::{Read, Write},
};

use toml::{Table, Value};
use winapi::{shared::windef::HWND__, um::winuser::MB_ICONERROR};

use crate::show_error;

use super::super::show_error_with_args;

#[allow(dead_code)]
pub enum Actions {
    DefaultProjectPath,
    NewProject,
}

pub fn set_property(action: Actions, path: &str, arg: &str, _hwnd: *mut HWND__) {
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
    let mut config = ss.parse::<Table>().unwrap_or_else(show_error_with_args!(
        toml::de::Error,
        "无法解析位于 {1} 的配置文件.\n原因:\n{0}",
        path
    ));

    match action {
        Actions::DefaultProjectPath => {
            config.insert(
                "default-project-path".into(),
                Value::String(arg.to_string()),
            );
            println!("{}", config.to_string());
        }
        Actions::NewProject => {
            let vec = config
                .get("projects")
                .unwrap_or_else(show_error!("Panic at get.panic"))
                .as_array()
                .unwrap_or_else(show_error!("Panic at as array"));
            let mut vec = vec.clone();
            vec.push(Value::String(arg.into()));
            config.insert("projects".into(), Value::Array(vec.to_vec()));
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
