Tauri 开发的高性能可扩展穿透管理客户端
扩展部分实现:
遍历 plugins 目录下中每个文件夹中的 manifest.json
manifest.json 与浏览器扩展属性类似包含: 版本 作者 权限 入口文件 等等
然后根据 manifest.json 中的入口文件做初始化, 并且在 