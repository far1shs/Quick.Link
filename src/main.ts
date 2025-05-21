import {createApp} from "vue";
import App from "./App.vue";
import "./index.css";
import {router} from "@/router/router.ts";
import {StoreInit} from "@/lib/store.ts";

const app = createApp(App);
app.use(router);

app.mount("#app");

document.addEventListener("dragstart", (e) => {
    e.preventDefault();
});
document.addEventListener("selectstart", (e) => {
    e.preventDefault();
});
document.addEventListener('contextmenu', (e) => {
    e.preventDefault();
});

StoreInit();