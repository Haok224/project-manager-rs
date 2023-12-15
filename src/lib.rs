pub mod settings;

#[macro_export]
macro_rules! show_error {
    ($message:expr , $($foramt_args:expr),+) => {
        || -> _ {
            unsafe {
                let message = CString::new(format!($message,$($foramt_args)+)).unwrap();
                let title = CString::new("ERROR").unwrap();
                winapi::um::winuser::MessageBoxA(
                    core::ptr::null_mut(),
                    message.as_ptr(),
                    title.as_ptr(),
                    MB_ICONERROR,
                )
            };
            panic!("Cannot read config file.");
        }
    };
    ($message:expr)=>{
        || -> _ {
            unsafe {
                let message = CString::new($message).unwrap();
                let title = CString::new("ERROR").unwrap();
                winapi::um::winuser::MessageBoxA(
                    core::ptr::null_mut(),
                    message.as_ptr(),
                    title.as_ptr(),
                    MB_ICONERROR,
                )
            };
            panic!("Cannot read config file.");
        }
    }
}

#[macro_export]
macro_rules! show_error_with_args {
    ($error_type:path,$message:expr , $($foramt_args:expr),+) => {
        |e:$error_type| -> _ {
            unsafe {
                let message = CString::new(format!($message,e,$($foramt_args)+)).unwrap();
                let title = CString::new("ERROR").unwrap();
                winapi::um::winuser::MessageBoxA(
                    core::ptr::null_mut(),
                    title.as_ptr(),
                    message.as_ptr(),
                    MB_ICONERROR,
                )
            };
            panic!("Cannot read config file.");
        }
    };
    ($error_type:path,$message:expr)=>{
        |e:$error_type| -> _ {
            unsafe {
                let message = CString::new(format!($message,e)).unwrap();
                let title = CString::new("ERROR").unwrap();
                winapi::um::winuser::MessageBoxA(
                    core::ptr::null_mut(),
                    title.as_ptr(),
                    message.as_ptr(),
                    MB_ICONERROR,
                )
            };
            panic!("Cannot read config file.");
        }
    }
}
