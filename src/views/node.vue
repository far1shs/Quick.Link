<template>
  <div v-if="data.node?.length == 0"
       style="height: calc(100vh - 80px)"
       class="flex items-center justify-center flex-col text-[14px] gap-2">
    <IconLowDensityRounded class="w-12 h-12"/>
    <span>这啥也没有右上角创建节点</span>
  </div>

  <NodeContextMenu v-else>
    <div class="flex flex-col gap-1.5">
      <div v-for="item in data.node"
           @contextmenu="emitter.emit('node_contextmenu', {label: data.label, item: item})"
           class="h-14 rounded-md px-3 py-2 transition-colors duration-250 hover:bg-[var(--sidebar-accent)] flex justify-between items-center group">
        <div class="flex flex-col">
          <span style="font-size: 14px">{{ item.name }}</span>
          <span style="margin-top: -4px; font-size: 12px" class="opacity-35">
          {{ item.ip }}:{{ item.port }}</span>
        </div>
        <div class="flex items-center gap-2">
          <div class="w-5 h-5 p-[2px]"><IconKeyRounded v-if="item.token"/></div>
          <IconPersonOutlineRounded v-if="item.user"  class="w-5 h-5 p-[2px]"/>
        </div>
      </div>
    </div>
  </NodeContextMenu>
</template>

<script setup lang="ts">
import {
  IconLowDensityRounded,
  IconKeyRounded,
  IconPersonOutlineRounded,
} from "@iconify-prerendered/vue-material-symbols";
import NodeContextMenu from "@/components/context-menu/NodeContextMenu.vue";
import {onMounted, onUnmounted, reactive} from "vue";
import {Store} from "@tauri-apps/plugin-store";
import {INode} from "@/type/node.ts";
import {AppTitleAction} from "@/model.ts";
import NodeAction from "@/components/action/NodeAction.vue";
import {emitter} from "@/lib/eventBus.ts";
import {ILabel} from "@/type/label.ts";

const data = reactive<{
  label: ILabel[] | null,
  node: INode[] | null,
}>({
  label: null,
  node: null,
});

onMounted(async () => {
  AppTitleAction.value = NodeAction;

  const store = await Store.load("node.json");
  data.label = await store.get("label") as ILabel[];
  data.node = await store.get("node") as INode[];

  emitter.on("node_update", async (value) => {
    if (value) {
      const _data = await store.get("node") as INode[];
      data.node = value.label == -1 ? _data : _data.filter((_item) => _item.label == value.label);

    } else data.node = await store.get("node") as INode[];
  });
})
onUnmounted(() => emitter.off("node_update"));
</script>