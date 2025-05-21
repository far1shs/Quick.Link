import {Component, ref, shallowRef} from "vue";

export const AppTitle = ref("");
export const AppSideBarSelect = ref("");
export const AppSideBarShow = ref(true);
export const AppTitleAction = shallowRef<Component | null>(null);

