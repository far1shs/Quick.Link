import {createRouter, createWebHistory} from "vue-router";
import {routes} from "./routes.ts";
import {AppSideBarSelect, AppTitle, AppTitleAction} from "@/model.ts";

export const router = createRouter({
    history: createWebHistory(),
    routes
})

router.beforeEach((to) => {
    AppTitle.value = to.meta.title as string;
    AppTitleAction.value = null;

    AppSideBarSelect.value = to.meta.value as string;
});