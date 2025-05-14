<template>
  <div v-if="data.tunnel?.length == 0"
       style="height: calc(100vh - 80px)"
       class="flex items-center justify-center flex-col text-[14px] gap-2">
    <IconLowDensityRounded class="w-12 h-12"/>
    <span>这啥也没有右上角创建隧道</span>
  </div>

  <TunnelContextMenu v-else>
    <div class="flex flex-col gap-1.5">
      <RouterLink v-for="item in data.tunnel" :to="{
          path: `/tunnel-view`,
          query: {
            data: encodeURIComponent(JSON.stringify(item))
          }
        }" custom v-slot="{navigate}">
        <div @click="navigate"
             @contextmenu="emitter.emit('tunnel_contextmenu', {label: data.label, item: item})"
             class="h-14 rounded-md px-3 py-2 transition-colors duration-250 hover:bg-[var(--sidebar-accent)] flex justify-between items-center group">
          <div class="flex flex-col">
            <span style="font-size: 14px">{{ item.name }}</span>
            <span style="margin-top: -4px; font-size: 12px" class="opacity-35">
              {{ item.protocol.toUpperCase() }} {{item.node_name}} - {{ item.node_id}} {{ item.ip }}:{{ item.port }} - {{ item.remote_port }}
            </span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-5 h-5 p-[2px]">
              <IconKeyRounded v-if="item.encrypt"/>
            </div>
            <IconCollapseContentRounded v-if="item.compress" class="w-5 h-5"/>
            <Switch v-model="item.status" @click.stop="editTunnelStatusClick(item)"/>
          </div>
        </div>
      </RouterLink>
    </div>
  </TunnelContextMenu>
</template>

<script setup lang="ts">
import {onMounted, reactive} from "vue";
import {AppTitleAction} from "@/model.ts";
import TunnelContextMenu from "@/components/context-menu/TunnelContextMenu.vue";
import TunnelAction from "@/components/action/TunnelAction.vue";
import {Store} from "@tauri-apps/plugin-store";
import {emitter} from "@/lib/eventBus.ts";
import {ILabel} from "@/type/label.ts";
import {ITunnel} from "@/type/tunnel.ts";
import {
  IconKeyRounded,
  IconLowDensityRounded,
  IconCollapseContentRounded,
} from "@iconify-prerendered/vue-material-symbols";
import {Switch} from "@/components/ui/switch";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "vue-sonner";

const data = reactive<{
  label: ILabel[] | null,
  tunnel: ITunnel[] | null,
}>({
  label: null,
  tunnel: null,
});

async function editTunnelStatusClick(value: ITunnel) {
  const store = await Store.load("tunnel.json", {
    autoSave: true,
  });

  const data = await store.get("tunnel") as ITunnel[];
  const dataFind = data.find((item) => item.id == value.id);
  if (dataFind) {
    dataFind.status = value.status;

    if (value.status) {
      let config: string[] = [
        value.protocol,
        "-n",
        value.name,
        "-i",
        value.ip,
        "-l",
        value.port!.toString(),
        "-s",
        value.node_ip,
        "-P",
        value.node_port.toString(),
        "-r",
        value.remote_port!.toString(),
      ];

      if (value.domain) config.push("-d", value.domain);
      if (value.encrypt) config.push("--ue");
      if (value.compress) config.push("--uc");

      const res: { status: boolean; message: string } = await invoke("run_frp", {
        id: value.id,
        config: config.join(" ")
      });

      if (res.status) toast.success(res.message);
      else {
        dataFind.status = !value.status;
        toast.error(res.message);
      }
    } else {
      const res: { status: boolean; message: string } = await invoke("stop_frp", {id: value.id});

      if (res.status) toast.success(res.message);
      else {
        toast.error(res.message);
      }
    }

    await store.set("tunnel", data);

    emitter.emit("tunnel_update");
  }
}

onMounted(async () => {
  AppTitleAction.value = TunnelAction;

  const store = await Store.load("tunnel.json");
  data.label = await store.get("label") as ILabel[];
  data.tunnel = await store.get("tunnel") as ITunnel[];

  emitter.on("tunnel_update", async (value) => {
    if (value) {
      const _data = await store.get("tunnel") as ITunnel[];
      data.tunnel = value.label == -1 ? _data : _data.filter((_item) => _item.label == value.label);

    } else data.tunnel = await store.get("tunnel") as ITunnel[];
  });
});
</script>