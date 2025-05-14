import {Component, ref, shallowRef} from "vue";

export const AppTitle = ref("");
export const AppSideBarSelect = ref("");
export const AppTitleAction = shallowRef<Component | null>(null);

