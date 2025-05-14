<template>
  <ContextMenu>
    <ContextMenuTrigger>
      <slot/>
    </ContextMenuTrigger>
    <ContextMenuContent class="w-52">
      <ContextMenuItem @click="copyClick">
        <IconContentCopyOutlineRounded/>
        复制地址
      </ContextMenuItem>
      <ContextMenuItem>
        <IconEditOutlineRounded/>
        编辑
      </ContextMenuItem>
      <ContextMenuSub>
        <ContextMenuSubTrigger>
          <IconSellOutline/>
          <span style="margin-left: 8px">标签</span>
        </ContextMenuSubTrigger>
        <ContextMenuSubContent class="w-48">
          <ContextMenuItem @click="editTunnelLabel(item!.id, 0)" :inset="item!.label != 0">
            <IconCheckRounded v-if="item!.label == 0"/>
            默认
          </ContextMenuItem>
          <ContextMenuItem v-for="_item in label"
                           @click="editTunnelLabel(item!.id, _item.value)"
                           :inset="item!.label != _item.value">
            <IconCheckRounded v-if="item!.label == _item.value"/>
            {{ _item.label }}
          </ContextMenuItem>
          <ContextMenuSeparator/>
          <ContextMenuItem>
            <IconSettingsOutlineRounded/>
            管理
          </ContextMenuItem>
        </ContextMenuSubContent>
      </ContextMenuSub>
      <ContextMenuItem @click="dialogOpen.del = true">
        <IconDeleteOutlineRounded/>
        删除
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>

  <Dialog v-model:open="dialogOpen.del">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>确认删除？</DialogTitle>
        <DialogDescription>
          你确定要删除名为 {{ item?.name }} - {{ item?.id }} 的隧道吗?
        </DialogDescription>
      </DialogHeader>

      <DialogFooter>
        <Button variant="destructive" @click="delTunnel">删除</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger
} from "@/components/ui/context-menu";
import {
  IconCheckRounded,
  IconContentCopyOutlineRounded,
  IconDeleteOutlineRounded,
  IconEditOutlineRounded,
  IconSellOutline,
  IconSettingsOutlineRounded
} from "@iconify-prerendered/vue-material-symbols";
import {onMounted, onUnmounted, reactive, ref} from "vue";
import {toast} from "vue-sonner";
import {Store} from "@tauri-apps/plugin-store";
import {emitter} from "@/lib/eventBus.ts";
import {ILabel} from "@/type/label.ts";
import {ITunnel} from "@/type/tunnel.ts";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {writeText} from "@tauri-apps/plugin-clipboard-manager";

const label = ref<ILabel[] | null>();
const item = ref<ITunnel | null>(null);

const dialogOpen = reactive({
  edit: false,
  label: false,
  del: false
});

async function copyClick() {
  await writeText(`${item.value?.node_ip}:${item.value?.remote_port}`);
  toast.success("复制成功")
}

async function editTunnelLabel(id: number, label: number) {
  const store = await Store.load("tunnel.json", {
    autoSave: true,
  });

  const data = await store.get("tunnel") as ITunnel[];
  const dataFind = data.find((_item) => _item.id == id);
  if (dataFind) {
    dataFind.label = label;
    await store.set("tunnel", data);
    emitter.emit("tunnel_update");
  }
}

async function delTunnel() {
  const store = await Store.load("tunnel.json", {
    autoSave: true,
  });

  const data = await store.get("tunnel") as ITunnel[];
  data.splice(data.findIndex((_item) => _item.id == item.value?.id), 1);
  await store.set("tunnel", data);

  dialogOpen.del = false;
  emitter.emit("tunnel_update");
  toast.success("删除成功");
}

onMounted(() => {
  emitter.on("tunnel_contextmenu", (value) => {
    item.value = {...value.item};
    label.value = value.label;
  });
});

onUnmounted(() => emitter.off("tunnel_contextmenu"));
</script>