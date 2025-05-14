import {getCurrentWindow} from "@tauri-apps/api/window";
import {Menu} from "@tauri-apps/api/menu";
import {TrayIcon} from "@tauri-apps/api/tray";

const appWindow = getCurrentWindow();

export async function initMenu() {
    return await Menu.new({
        items: [
            {
                id: "show",
                text: "显示",
                action: () => {
                    appWindow.show();
                    appWindow.unminimize();
                    appWindow.setFocus();
                },
            }, {
                id: "quit",
                text: "退出",
                action: () => {
                    appWindow.close();
                },
            },
        ],
    })
}
export async function TrayIconInit() {
    const menu = await initMenu();

    await TrayIcon.new({
        tooltip: "Quick Link",
        icon: "icons/32x32.png",
        menu: menu,
        menuOnLeftClick: false,
        action: (event: any) => {
            if (event.type == "Click" && event.button == "Left") {
                appWindow.show();
                appWindow.unminimize();
                appWindow.setFocus();
            }
        },
    });
}