#![windows_subsystem = "windows"]

use slint::LogicalPosition;

slint::include_modules!();
fn main() {

    let app = AppWindow::new().unwrap();

    let handle1 = app.as_weak();
    let handle2 = app.as_weak();
    app.global::<Functions>().on_close_window(move || {
        handle2.upgrade().unwrap().hide().unwrap();
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

    app.run().unwrap()
}
