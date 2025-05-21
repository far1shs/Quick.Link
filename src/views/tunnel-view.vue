<template>
  <div class="h-11 px-3 flex justify-between items-center group">
    <div class="flex flex-col text-[14px]">
      <span style="font-size: 14px">
        {{ data.protocol.toUpperCase() }} {{ data.ip }}:{{ data.port }} - {{ data.remote_port }}</span>
      <span style="margin-top: -4px; font-size: 12px" class="opacity-35">
        {{ data.node_name }} - {{ data.node_id }}
      </span>
    </div>
    <div class="flex items-center gap-2">
      <div class="w-5 h-5 p-[2px]">
        <IconKeyRounded v-if="data.encrypt"/>
      </div>
      <IconCollapseContentRounded v-if="data.compress" class="w-5 h-5"/>
      <Switch v-model="data.status" @click="editTunnelStatusClick"/>
    </div>
  </div>

  <div class="separator"/>

  <div v-if="!data.status"
       style="height: calc(100vh - 120px)"
       class="flex items-center justify-center flex-col text-[14px] gap-2">
    <IconPauseCircleOutlineRounded class="w-12 h-12"/>
    <span>隧道还没运行呢</span>
  </div>

  <div v-else-if="logs.length == 0"
       style="height: calc(100vh - 120px)"
       class="flex items-center justify-center flex-col text-[14px] gap-2">
    <IconLowDensityRounded class="w-12 h-12"/>
    <span>这里没有任何日志你清空了?</span>
  </div>

  <div v-else class="flex flex-col gap-0.5">
    <div v-for="item in logs">
      <ContextMenu>
        <ContextMenuTrigger>
          <div class="rounded-md px-3 py-1 transition-colors duration-250 hover:bg-[var(--sidebar-accent)] flex items-center text-[14px]">
            <span>{{ item }}</span>
          </div>
        </ContextMenuTrigger>
        <ContextMenuContent class="w-52">
          <ContextMenuItem @click="copyClick(item)">
            <IconContentCopyOutlineRounded/>
            复制
          </ContextMenuItem>
          <ContextMenuItem>
            <IconDeleteOutlineRounded/>
            清空
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  IconCollapseContentRounded, IconContentCopyOutlineRounded,
  IconKeyRounded,
  IconLowDensityRounded,
  IconPauseCircleOutlineRounded,
  IconDeleteOutlineRounded
} from "@iconify-prerendered/vue-material-symbols";
import {ITunnel} from "@/type/tunnel.ts";
import {Store} from "@tauri-apps/plugin-store";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "vue-sonner";
import {onMounted, onUnmounted, reactive, ref} from "vue";
import {AppTitle} from "@/model.ts";
import {Switch} from "@/components/ui/switch";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {writeText} from "@tauri-apps/plugin-clipboard-manager";

const data = reactive<ITunnel>(JSON.parse(decodeURIComponent(new URLSearchParams(window.location.search).get("data") as string)));
const logs = ref<string[]>([]);
let unlistenLogs: UnlistenFn;

async function editTunnelStatusClick() {
  const store = await Store.load("tunnel.json", {
    autoSave: true,
  });

  const _data = await store.get("tunnel") as ITunnel[];
  const dataFind = _data.find((item) => item.id == data.id);
  if (dataFind) {
    dataFind.status = data.status;

    if (data.status) {
      let config: string[] = [
          data.protocol,
          "-n",
          data.name,
          "-i",
          data.ip,
          "-l",
          data.port!.toString(),
          "-s",
          data.node_ip,
          "-P",
          data.node_port.toString(),
      ];

      if (data.remote_port) config.push("-r", data.remote_port.toString());
      if (data.domain) {
        data.domain.split(" ").forEach((item) => {
          config.push("-d", item);
        });
      }
      if (data.encrypt) config.push("--ue");
      if (data.compress) config.push("--uc");

      const res: { status: boolean; message: string } = await invoke("run_frp", {
        id: data.id,
        config: config.join(" ")
      });

      if (res.status) {
        await listenTunnelLogs();
        toast.success(res.message);
      }
      else {
        dataFind.status = !data.status;
        toast.error(res.message);
      }
    } else {
      const res: { status: boolean; message: string } = await invoke("stop_frp", {id: data.id});

      if (res.status) toast.success(res.message);
      else {
        toast.error(res.message);
      }

      if (unlistenLogs) unlistenLogs();
      logs.value = [];
    }

    data.status = dataFind.status;
    await store.set("tunnel", _data);
  }
}

async function copyClick(value: string) {
  await writeText(value);
  toast.success("复制成功")
}

async function listenTunnelLogs() {
  const res: {
    status: boolean,
    data: string[],
    message: string
  } = await invoke("get_frp_logs", {id: data.id});

  if (res.status) {
    logs.value = res.data;
    unlistenLogs = await listen<string>(`frp_logs_${data.id}`, (event) => {
      logs.value.push(event.payload);
    });
  }
  else toast.error(res.message);
}

onMounted(async () => {
  AppTitle.value = `${data.name} - ${data.id}`;

  if (data.status) listenTunnelLogs();
});

onUnmounted(() => {
  if (unlistenLogs) unlistenLogs();
});
</script>