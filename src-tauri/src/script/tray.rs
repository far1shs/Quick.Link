use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Error, Manager};

pub fn init(app: AppHandle) -> Result<(), Error> {
    // 获取 'static 生命周期的 AppHandle
    let app_handle = app.clone();

    // 创建菜单项
    let show_i = MenuItem::with_id(&app_handle, "show", "显示", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(&app_handle, "quit", "退出", true, None::<&str>)?;

    // 构建菜单
    let menu = Menu::with_items(&app_handle, &[&show_i, &quit_i])?;

    // 创建托盘图标
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Quick Link")
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |handle, event| match event.id.as_ref() {
            "show" => {
                println!("显示窗口");
                if let Some(window) = handle.get_window("main") {
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                }
            }
            "quit" => {
                println!("退出程序");
                handle.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(move |_tray, event| {
            if let TrayIconEvent::Click { button, .. } = event {
                if button == MouseButton::Left {
                    println!("托盘图标左键点击");

                    if let Some(window) = app.clone().get_window("main") {
                        window.show().unwrap();
                        window.unminimize().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }
        })
        .build(&app_handle)?;

    Ok(())
}
