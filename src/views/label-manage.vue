<template>
  <div v-if="data?.length == 0"
       style="height: calc(100vh - 80px)"
       class="flex items-center justify-center flex-col text-[14px] gap-2">
    <IconLowDensityRounded class="w-12 h-12"/>
    <span>这里啥也没有右上角添加一个标签</span>
  </div>

  <ContextMenu>
    <ContextMenuTrigger>
      <div v-for="item in data"
           @contextmenu="selectItemData = item"
           class="h-10 rounded-md px-3 py-2 transition-colors duration-250 hover:bg-[var(--sidebar-accent)] flex justify-between items-center group">
        <div class="flex flex-col">
          <span style="font-size: 14px">{{ item.label }}</span>
        </div>
      </div>
    </ContextMenuTrigger>
    <ContextMenuContent class="w-52">
      <ContextMenuItem @click="openEditClick">
        <IconEditOutlineRounded/>
        编辑
      </ContextMenuItem>
      <ContextMenuItem @click="dialogOpen.del = true">
        <IconDeleteOutlineRounded/>
        删除
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>

  <Dialog v-model:open="dialogOpen.edit">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>修改标签</DialogTitle>
      </DialogHeader>

      <Input v-model="editLabel!.label" placeholder="输入标签名"/>

      <DialogClose>
        <Button @click="editClick" class="mt-1 w-full">修改</Button>
      </DialogClose>
    </DialogContent>
  </Dialog>

  <Dialog v-model:open="dialogOpen.del">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>确认删除？</DialogTitle>
        <DialogDescription>
          你确定要删除名为 {{ selectItemData?.label }} - {{ selectItemData?.value }} 的表情吗?
        </DialogDescription>
      </DialogHeader>

      <DialogFooter>
        <Button variant="destructive" @click="delLabelClick">删除</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import {AppTitleAction} from "@/model.ts";
import LabelManageAction from "@/components/action/LabelManageAction.vue";
import {onMounted, onUnmounted, reactive, ref} from "vue";
import {emitter} from "@/lib/eventBus.ts";
import {Store} from "@tauri-apps/plugin-store";
import {ILabel} from "@/type/label.ts";
import {
  IconLowDensityRounded,
  IconEditOutlineRounded,
  IconDeleteOutlineRounded
} from "@iconify-prerendered/vue-material-symbols";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {Button} from "@/components/ui/button";
import {
  Dialog, DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import {toast} from "vue-sonner";
import {Input} from "@/components/ui/input";

const data = ref<ILabel[]>();
const type = ref<"tunnel" | "node">("tunnel");
const selectItemData = ref<ILabel>();
const editLabel = ref<ILabel>();
const dialogOpen = reactive({
  edit: false,
  del: false
})

async function openEditClick() {
  editLabel.value = JSON.parse(JSON.stringify(selectItemData.value));
  dialogOpen.edit = true;
}

async function editClick() {
  const store = await Store.load(`${type.value}.json`, {
    autoSave: true
  });

  const data = await store.get("label") as ILabel[];
  const dataIndex = data.findIndex((item) => item.value == selectItemData.value?.value);
  if (dataIndex != -1) {
    data[dataIndex] = editLabel.value!;
    await store.set("label", data);

    dialogOpen.edit = false;
    emitter.emit("label_manage_update", {
      type: type.value
    });
    toast.success("修改成功");
  }
}

async function delLabelClick() {
  const store = await Store.load(`${type.value}.json`, {
    autoSave: true
  });

  const data = await store.get("label") as ILabel[];
  data.splice(data.findIndex((item) => item.value == selectItemData.value?.value), 1);
  await store.set("label", data);

  dialogOpen.del = false;
  emitter.emit("label_manage_update", {type: type.value});
  toast.success("删除成功");
}

onMounted(async () => {
  AppTitleAction.value = LabelManageAction;

  const store = await Store.load("tunnel.json");
  data.value = await store.get("label") as ILabel[];

  emitter.on("label_manage_update", async (value) => {
    const store = await Store.load(`${value.type}.json`);
    data.value = await store.get("label") as ILabel[];
    type.value = value.type;
  });
});

onUnmounted(() => emitter.off("label_manage_update"));
</script>