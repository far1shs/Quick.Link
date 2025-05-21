<template>
  <ContextMenu>
    <ContextMenuTrigger>
      <slot/>
    </ContextMenuTrigger>
    <ContextMenuContent class="w-52">
      <ContextMenuItem @click="dialogOpen.edit = true">
        <IconEditOutlineRounded/>
        编辑
      </ContextMenuItem>
      <ContextMenuSub>
        <ContextMenuSubTrigger>
          <IconSellOutline/>
          <span style="margin-left: 8px">标签</span>
        </ContextMenuSubTrigger>
        <ContextMenuSubContent class="w-48">
          <ContextMenuItem @click="editNodeLabelClick(item!.id, 0)" :inset="item!.label != 0">
            <IconCheckRounded v-if="item!.label == 0"/>
            默认
          </ContextMenuItem>
          <ContextMenuItem v-for="_item in label"
                           @click="editNodeLabelClick(item!.id, _item.value)"
                           :inset="item!.label != _item.value">
            <IconCheckRounded v-if="item!.label == _item.value"/>
            {{ _item.label }}
          </ContextMenuItem>
          <ContextMenuSeparator/>
          <RouterLink :to="{ path: '/label-manage', query: { type: 'node' } }">
            <ContextMenuItem>
              <IconSettingsOutlineRounded/>
              管理
            </ContextMenuItem>
          </RouterLink>
        </ContextMenuSubContent>
      </ContextMenuSub>
      <ContextMenuItem @click="dialogOpen.del = true">
        <IconDeleteOutlineRounded/>
        删除
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>

  <Dialog v-model:open="dialogOpen.edit">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>修改节点</DialogTitle>
      </DialogHeader>

      <Accordion type="single" class="w-full" collapsible default-value="default">
        <AccordionItem value="default">
          <AccordionTrigger>基础</AccordionTrigger>
          <AccordionContent>
            <div class="space-y-4 px-[1px]">
              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="name">名称</Label>
                <Input v-model="item!.name" id="name" placeholder="名称"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="ip">IP</Label>
                <Input v-model="item!.ip" id="ip" placeholder="IP"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="port">端口</Label>
                <Input v-model="item!.port" id="port" placeholder="端口"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="token">token</Label>
                <Input v-model="item!.token" id="token" placeholder="token"/>
              </div>
            </div>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="senior">
          <AccordionTrigger>高级</AccordionTrigger>
          <AccordionContent>
            <div class="space-y-4 px-[1px]">
              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="user">user</Label>
                <Input v-model="item!.user" id="user" placeholder="user"/>
              </div>
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>

      <DialogFooter>
        <Button @click="editNodeClick(item!.id)" size="sm" class="w-full">
          修改
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <Dialog v-model:open="dialogOpen.del">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>确认删除？</DialogTitle>
        <DialogDescription>
          你确定要删除名为 {{ item?.name }} - {{ item?.id }} 的节点吗?
        </DialogDescription>
      </DialogHeader>

      <DialogFooter>
        <Button variant="outline">更换节点后删除</Button>
        <Button variant="destructive" @click="delNodeClick">直接删除</Button>
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
  IconDeleteOutlineRounded,
  IconEditOutlineRounded,
  IconSellOutline,
  IconSettingsOutlineRounded
} from "@iconify-prerendered/vue-material-symbols";
import {onMounted, onUnmounted, reactive, ref} from "vue";
import {INode} from "@/type/node.ts";
import {emitter} from "@/lib/eventBus.ts";
import {Store} from "@tauri-apps/plugin-store";
import {Button} from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import {Accordion, AccordionContent, AccordionItem, AccordionTrigger} from "@/components/ui/accordion";
import {Input} from "@/components/ui/input";
import {Label} from "@/components/ui/label";
import {toast} from "vue-sonner";
import {ILabel} from "@/type/label.ts";

const label = ref<ILabel[] | null>();
const item = ref<INode | null>(null);

const dialogOpen = reactive({
  edit: false,
  del: false
});

async function editNodeClick(id: number) {
  const store = await Store.load("node.json", {
    autoSave: true,
  });

  const data = await store.get("node") as INode[];
  const dataIndex = data.findIndex((_item) => _item.id == id);
  if (dataIndex != -1) {
    data[dataIndex] = item.value!;
    await store.set("node", data);

    dialogOpen.edit = false;
    emitter.emit("node_update");
    toast.success("修改成功");
  }
}

async function editNodeLabelClick(id: number, label: number) {
  const store = await Store.load("node.json", {
    autoSave: true,
  });

  const data = await store.get("node") as INode[];
  const dataFind = data.find((_item) => _item.id == id);
  if (dataFind) {
    dataFind.label = label;
    await store.set("node", data);
    emitter.emit("node_update");
  }
}

async function delNodeClick() {
  const store = await Store.load("node.json", {
    autoSave: true,
  });

  const data = await store.get("node") as INode[];
  data.splice(data.findIndex((_item) => _item.id == item.value?.id), 1);
  await store.set("node", data);

  dialogOpen.del = false;
  emitter.emit("node_update");
  toast.success("删除成功");
}

onMounted(() => {
  emitter.on("node_contextmenu", (value) => {
    item.value = {...value.item};
    label.value = value.label;
  });
})

onUnmounted(() => emitter.off("node_contextmenu"));
</script>