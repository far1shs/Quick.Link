export const routes = [
    {
        path: "/",
        redirect: "/tunnel",
    }, {
        path: "/tunnel",
        component: () => import("@/views/tunnel.vue"),
        meta: { title: "隧道", value: "tunnel" },
    }, {
        path: "/tunnel-add",
        component: () => import("@/views/tunnel-add.vue"),
        meta: { title: "创建隧道 - 节点选择", value: "tunnel" },
    }, {
        path: "/tunnel-view",
        component: () => import("@/views/tunnel-view.vue"),
        meta: { title: "隧道详情", value: "tunnel" },
    }, {
        path: "/node",
        component: () => import("@/views/node.vue"),
        meta: { title: "节点", value: "node" },
    }, {
        path: "/plugin",
        component: () => import("@/views/plugin.vue"),
        meta: { title: "插件", value: "plugin" },
    }, {
        path: "/plugin-view",
        component: () => import("@/views/plugin-view.vue"),
        meta: { title: "插件详情", value: "plugin" },
    }, {
        path: "/plugin-install",
        component: () => import("@/views/plugin-install.vue"),
        meta: { title: "插件安装", value: "plugin" },
    }, {
        path: "/settings",
        component: () => import("@/views/settings.vue"),
        meta: { title: "设置", value: "settings" },
    }
];