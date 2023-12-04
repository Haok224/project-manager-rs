//#![windows_subsystem="windows"]

use std::{
    error::Error,
    ffi::CString,
    io::{Read, Write},
    path::Path,
    process::exit,
};

use slint::{LogicalPosition, SharedString};
use toml::Table;
use winapi::um::winuser::{FindWindowA, MB_ICONERROR, MB_OK};

slint::include_modules!();
fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path;
    let mut config = Table::new();
    let mut has_config_file = false;
    //判断文件是否存在
    let current_path = Path::new("prjmng.toml");
    let appdata_path = format!(
        "{}\\ProjectManager\\prjmng.toml",
        std::env::var("AppData").unwrap()
    );
    if current_path.exists() && current_path.is_file() {
        has_config_file = true;
        //读取当前目录的配置文件
        let file = std::fs::File::open(current_path);
        //发生错误，弹出错误提示框并panic
        if let Err(_) = file {
            unsafe {
                let message = CString::new("ERROR").unwrap();
                let title = CString::new("无法打开位于当前目录的配置文件.").unwrap();
                winapi::um::winuser::MessageBoxA(
                    core::ptr::null_mut(),
                    message.as_ptr(),
                    title.as_ptr(),
                    MB_ICONERROR,
                )
            };
            panic!("Cannot open config file.");
        } else {
            config_file_path = "prjmng.toml".to_string();
            let mut config_str = String::new();
            let mut file = file.unwrap();
            file.read_to_string(&mut config_str)
                .unwrap_or_else(|_| -> _ {
                    unsafe {
                        let message = CString::new("无法在当前目录读取配置文件.").unwrap();
                        let title = CString::new("ERROR").unwrap();
                        winapi::um::winuser::MessageBoxA(
                            core::ptr::null_mut(),
                            message.as_ptr(),
                            title.as_ptr(),
                            MB_ICONERROR,
                        )
                    };
                    panic!("Cannot read config file.");
                });
            unsafe {
                config = config_str.parse::<Table>().unwrap_or_else(|err| -> _ {
                    let message = CString::new(format!(
                        "无法在当前目录读取配置文件.\n原因:\n{}",
                        err.to_string()
                    ))
                    .unwrap();
                    let title = CString::new("ERROR").unwrap();
                    winapi::um::winuser::MessageBoxA(
                        core::ptr::null_mut(),
                        message.as_ptr(),
                        title.as_ptr(),
                        MB_ICONERROR,
                    );
                    panic!("Cannot read config file.\nCaused by:\n{}", err.to_string());
                })
            };
            drop(file);
        }
    } else {
        //读取位于AppData的配置文件

        config_file_path = appdata_path.clone();
        let appdata_path = Path::new(&appdata_path);
        if appdata_path.exists() && appdata_path.is_file() {
            has_config_file = true;
            let file = std::fs::File::open(appdata_path);
            if let Err(_) = file {
                unsafe {
                    let message = CString::new(
                        "无法打开位于`%AppData%\\ProjectManager\\prjmng.toml`的配置文件.",
                    )
                    .unwrap();
                    let title = CString::new("ERROR").unwrap();
                    winapi::um::winuser::MessageBoxA(
                        core::ptr::null_mut(),
                        message.as_ptr(),
                        title.as_ptr(),
                        MB_ICONERROR,
                    )
                };
                panic!("Cannot open config file.");
            } else {
                let mut config_str = String::new();
                let mut file = file.unwrap();
                file
                    .read_to_string(&mut config_str)
                    .unwrap_or_else(|err| -> _ {
                        unsafe {
                            let message = CString::new(
                                format!("无法读取位于`%AppData%\\ProjectManager\\prjmng.toml`的配置文件.\n原因:\n{}",err.to_string()),
                            )
                            .unwrap();
                            let title = CString::new("ERROR").unwrap();
                            winapi::um::winuser::MessageBoxA(
                                core::ptr::null_mut(),
                                message.as_ptr(),
                                title.as_ptr(),
                                MB_ICONERROR,
                            )
                        };
                        panic!("Cannot read config file.\nCaused by:\n{}",err.to_string());
                    });
                unsafe {
                    config = config_str.parse::<Table>().unwrap_or_else(|err| -> _ {
                        let message = CString::new(format!("无法打开位于`%AppData%\\ProjectManager\\prjmng.toml`的配置文件.\n原因:\n{}",err.to_string())).unwrap();
                        let title = CString::new("ERROR").unwrap();
                        winapi::um::winuser::MessageBoxA(
                            core::ptr::null_mut(),
                            message.as_ptr(),
                            title.as_ptr(),
                            MB_ICONERROR,
                        );
                        panic!("Cannot read config file.\nCaused by:\n{}",err.to_string());
                    })
                };
                drop(file);
            }
        }
    }

    //如果不存在配置文件，新建
    if !has_config_file {
        //默认在当前目录创建新文件
        let mut file =
            std::fs::File::create("prjmng.toml").unwrap_or_else(|err| -> std::fs::File {
                unsafe {
                    let message = CString::new(format!(
                        "无法在当前目录创建新的配置文件.\n原因:\n{}",
                        err.to_string()
                    ))
                    .unwrap();
                    let title = CString::new("ERROR").unwrap();
                    winapi::um::winuser::MessageBoxA(
                        core::ptr::null_mut(),
                        message.as_ptr(),
                        title.as_ptr(),
                        MB_ICONERROR,
                    )
                };
                panic!(
                    "Cannot create config file.\nCaused by:\n{}",
                    err.to_string()
                );
            });
        file.write_all("default-project-path = \"projects\"".as_bytes())
            .unwrap_or_else(|err| -> _ {
                unsafe {
                    let message =
                        CString::new(format!("无法写入新的配置文件.原因:\n{}\n", err.to_string()))
                            .unwrap();
                    let title = CString::new("ERROR").unwrap();
                    winapi::um::winuser::MessageBoxA(
                        core::ptr::null_mut(),
                        message.as_ptr(),
                        title.as_ptr(),
                        MB_ICONERROR,
                    )
                };
                panic!(
                    "Cannot write to config file.\nCaused by:\n{}",
                    err.to_string()
                );
            });
        unsafe {
            let message = CString::new("重新运行以开始Project Manager.").unwrap();
            let title = CString::new("INFO").unwrap();
            winapi::um::winuser::MessageBoxA(
                core::ptr::null_mut(),
                message.as_ptr(),
                title.as_ptr(),
                MB_OK,
            )
        };
        exit(0);
    }

    let app = AppWindow::new().unwrap();

    let handle1 = app.as_weak();
    let handle2 = app.as_weak();
    app.global::<Functions>().on_close_window(move || {
        handle2.upgrade().unwrap().hide().unwrap();
    });

    app.global::<Functions>().on_create_project(
        |name: SharedString, des: SharedString, path: SharedString, lang: SharedString| {
            println!("{name} {des} {path} {lang}");
        },
    );

    let config_for_browse = config.clone();

    let config_for_get_df_prj = config.clone();
    app.global::<Functions>()
        .on_get_default_prj_path(move || -> SharedString {
            SharedString::from(
                config_for_get_df_prj
                    .get("default-project-path")
                    .unwrap()
                    .as_str()
                    .unwrap(),
            )
        });

    app.on_move_window(move |offset_x, offset_y| {
        let main = handle1.upgrade().unwrap();
        //获取窗口的物理坐标
        let logical_pos = main
            .window()
            .position()
            .to_logical(main.window().scale_factor());
        //窗口坐标添加上偏移量，设置为新的位置
        main.window().set_position(LogicalPosition::new(
            logical_pos.x + offset_x,
            logical_pos.y + offset_y,
        ));
    });
    app.show()?;
    let title = CString::new("Project Manager")?;
    let window: *mut winapi::shared::windef::HWND__ =
        unsafe { FindWindowA(std::ptr::null_mut(), title.as_ptr()) };
    println!("{}", window as isize);
    app.global::<Functions>()
        .on_browse_default_project_path(move || -> SharedString {
            let res = nfd::open_pick_folder(None);
            if let Result::Ok(status) = res {
                if let nfd::Response::Okay(path) = status {
                    project_manager::settings::config::set_default_path(
                        &path,
                        &config_file_path,
                        window,
                    );
                    return SharedString::from(&path);
                }
            }
            return SharedString::from(
                config_for_browse
                    .get("default-project-path")
                    .unwrap()
                    .as_str()
                    .unwrap(),
            );
        });
    slint::run_event_loop()?;
    app.hide()?;
    Ok(())
}
