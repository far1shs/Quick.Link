# 介绍
Tauri 实现的跨平台 "FRP" 图形化客户端, 使用的 FRP 为 Far1sh 二次汉化版本

目标是做一个好用, 易用的 **穿透** 跨平台客户端 通过扩展连接在线服务商, 统一管理

官网 [Far1sh Release](https://release.far1sh.icu/app/quick_link) 里面还有其他小项目

# 功能
目前已经能够正常简单使用
- [ ] 账号(云同步等)
- [x] 隧道管理
- [x] 节点管理
- [ ] 扩展
- [ ] 拖放导入
- [ ] 文件关联
- [ ] 超级链接
- [ ] 更简洁的源码
- [ ] 隧道删除(完整)
- [ ] 标签删除(完整)

可能还会加入其他功能

# 存在问题
节点 user 无法生效，此问题是我粗心造成的 \
优化逻辑目前的隧道中混杂这节点信息, 脱离预期, 将在下一个次要版本中修复