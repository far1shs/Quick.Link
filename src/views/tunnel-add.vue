<template>
  <div v-if="data.node?.length == 0"
       style="height: calc(100vh - 80px)"
       class="flex items-center justify-center flex-col text-[14px] gap-2">
    <IconLowDensityRounded class="w-12 h-12"/>
    <span>这啥也没有快去创建节点</span>
  </div>

  <div class="flex flex-col gap-1.5">
    <div v-for="item in data.node"
         @click="tunnelDialogOpenClick(item)"
         class="h-14 rounded-md px-3 py-2 transition-colors duration-250 hover:bg-[var(--sidebar-accent)] flex justify-between items-center group">
      <div class="flex flex-col">
        <span style="font-size: 14px">{{ item.name }}</span>
        <span style="margin-top: -4px; font-size: 12px" class="opacity-35">
          {{ item.ip }}:{{ item.port }}</span>
      </div>
      <div class="flex items-center gap-2">
        <div class="w-5 h-5 p-[2px]">
          <IconKeyRounded v-if="item.token"/>
        </div>
        <IconPersonOutlineRounded v-if="item.user"/>
      </div>
    </div>
  </div>

  <Dialog v-model:open="newTunnelDialogOpen">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle class="flex items-center flex-2">选择协议</DialogTitle>
      </DialogHeader>

      <div class="flex flex-col gap-2">
        <div v-for="item in tunnelProtocol"
             @click="protocolClick(item.value)"
             class="h-10 rounded-md px-3 py-2 transition-colors duration-250 hover:bg-[var(--sidebar-accent)] w-full">
          <span class="text-[14px]">{{ item.label }}</span>
        </div>
      </div>
    </DialogContent>
  </Dialog>

  <TcpUdpTunnelDialog/>
  <HTTPTunnelDialog/>
</template>

<script setup lang="ts">
// 这里大概与 node.vue 相同可以说就是复制了稍微改了改
import {
  IconLowDensityRounded,
  IconKeyRounded,
  IconPersonOutlineRounded,
} from "@iconify-prerendered/vue-material-symbols";
import {onMounted, onUnmounted, reactive, ref} from "vue";
import {Store} from "@tauri-apps/plugin-store";
import {INode} from "@/type/node.ts";
import {AppTitleAction} from "@/model.ts";
import {emitter} from "@/lib/eventBus.ts";
import {ILabel} from "@/type/label.ts";
import {tunnelProtocol} from "@/type/tunnel.ts";
import {Dialog, DialogContent, DialogHeader, DialogTitle} from "@/components/ui/dialog";
import TunnelAddAction from "@/components/action/TunnelAddAction.vue";
import TcpUdpTunnelDialog from "@/components/dialog/TcpUdpTunnelDialog.vue";
import HTTPTunnelDialog from "@/components/dialog/HTTPTunnelDialog.vue";

const newTunnelDialogOpen = ref(false);

const data = reactive<{
  label: ILabel[] | null,
  node: INode[] | null,
}>({
  label: null,
  node: null,
});
let selectData = reactive({
  protocol: "",
  node_id: 0,
  node_name: "",
  node_ip: "",
  node_port: 0,
});

function tunnelDialogOpenClick(value: INode) {
  newTunnelDialogOpen.value = true;

  selectData = {
    ...selectData,
    node_id: value.id,
    node_name: value.name,
    node_ip: value.ip,
    node_port: value.port
  };
}

function protocolClick(value: string) {
  newTunnelDialogOpen.value = false;
  selectData.protocol = value;

  if (value == "tcp" || value == "udp") {
    emitter.emit("tunnel_add_tcp_udp", selectData);
  } else if (value == "http" || value == "https") {
    emitter.emit("tunnel_add_http", selectData);
  }
}

onMounted(async () => {
  AppTitleAction.value = TunnelAddAction;

  const store = await Store.load("node.json");
  data.label = await store.get("label") as ILabel[];
  data.node = await store.get("node") as INode[];

  emitter.on("tunnel_add_update", async (value) => {
    if (value) {
      const _data = await store.get("node") as INode[];
      data.node = value.label == -1 ? _data : _data.filter((_item) => _item.label == value.label);
    } else data.node = await store.get("node") as INode[];
  });
});

onUnmounted(() => emitter.off("tunnel_add_update"));
</script>