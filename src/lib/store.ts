import {Store} from "@tauri-apps/plugin-store";
import {useColorMode} from "@vueuse/core";

async function tunnel() {
    const store = await Store.load("tunnel.json", {
        autoSave: true
    });

    if (!await store.get("label")) await store.set("label", [])
    if (!await store.get("tunnel")) await store.set("tunnel", []);
}

async function node() {
    const store = await Store.load("node.json", {
        autoSave: true
    });

    if (!await store.get("label")) await store.set("label", [])
    if (!await store.get("node")) await store.set("node", []);
}

async function settings() {
    const store = await Store.load("settings.json", {
        autoSave: true
    });

    useColorMode().value =
        await store.get("theme") ? await store.get("theme") as "light" | "dark" | "auto" : "auto";
}

export const StoreInit = async () => {
    await tunnel();
    await node();
    await settings();
};