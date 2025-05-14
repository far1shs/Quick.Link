<template>
  <Dialog v-model:open="newNodeDialogOpen">
    <DialogTrigger>
      <Button variant="ghost" class="w-7 h-7">
        <IconAddRounded/>
      </Button>
    </DialogTrigger>
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>创建节点</DialogTitle>
      </DialogHeader>

      <Accordion type="single" class="w-full" collapsible default-value="default">
        <AccordionItem value="default">
          <AccordionTrigger>基础</AccordionTrigger>
          <AccordionContent>
            <div class="space-y-4 px-[1px]">
              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="name">名称</Label>
                <Input v-model="newNodeData.name" id="name" placeholder="名称"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="ip">IP</Label>
                <Input v-model="newNodeData.ip" id="ip" placeholder="IP"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="port">端口</Label>
                <Input v-model="newNodeData.port"
                       id="port"
                       placeholder="端口"
                       oninput="this.value = this.value.replace(/[^0-9]/g, '').slice(0, 5)"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="token">token</Label>
                <Input v-model="newNodeData.token" id="token" placeholder="token"/>
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
                <Input v-model="newNodeData.user" id="user" placeholder="user"/>
              </div>
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>

      <DialogFooter>
        <Button @click="newNodeClick" size="sm" class="w-full">
          创建
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <DropdownMenu>
    <DropdownMenuTrigger>
      <Button variant="ghost" class="h-7 px-2 text-[13px]">
        <span>{{ label?.find((item: ILabel) => item!.value == labelSelect)?.label }}</span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-48">
      <DropdownMenuItem v-for="item in label"
                        @click="labelSelectClick(item!.value)"
                        :inset="labelSelect != item!.value">
        <IconCheckRounded v-if="labelSelect == item!.value"/>
        {{ item!.label }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>

<script setup lang="ts">
import {IconAddRounded, IconCheckRounded} from "@iconify-prerendered/vue-material-symbols";
import {Button} from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {Label} from "@/components/ui/label";
import {Input} from "@/components/ui/input";
import {Accordion, AccordionItem, AccordionTrigger, AccordionContent} from "@/components/ui/accordion";
import {onMounted, reactive, ref, watch} from "vue";
import {INode} from "@/type/node.ts";
import {Store} from "@tauri-apps/plugin-store";
import {emitter} from "@/lib/eventBus.ts";
import {toast} from "vue-sonner";
import {ILabel} from "@/type/label.ts";

const newNodeDialogOpen = ref(false);
let newNodeData = reactive<INode>({
  id: generate8DigitRandomNumber(),
  name: `QLN-${generate8DigitRandomNumber()}`,
  label: 0,
  ip: "",
  port: 7000,
  user: "",
  token: "",
});

const label = ref<ILabel[] | null>(null);
const labelSelect = ref(-1);

async function newNodeClick() {
  const store = await Store.load("node.json", {
    autoSave: true,
  });

  const data = await store.get("node") as INode[];

  while (true) {
    const dataFind = data.find((item) => item.id == newNodeData.id)
    if (dataFind) newNodeData.id = generate8DigitRandomNumber();
    else break;
  }

  data.push(newNodeData);

  await store.set("node", data);
  newNodeDialogOpen.value = false;
  emitter.emit("node_update");
  toast.success("创建成功");
}

function labelSelectClick(id: number) {
  labelSelect.value = id;
  emitter.emit("node_update", {
    label: labelSelect.value,
  });
}

function generate8DigitRandomNumber(): number {
  return Math.floor(10000000 + Math.random() * 90000000);
}

onMounted(async () => {
  watch(() => newNodeDialogOpen.value, async () => {
    if (!newNodeDialogOpen.value) {
      newNodeData = reactive<INode>({
        id: generate8DigitRandomNumber(),
        name: `QLN-${generate8DigitRandomNumber()}`,
        label: 0,
        ip: "",
        port: 7000,
        user: "",
        token: "",
      });
    }
  })

  const store = await Store.load("node.json");
  label.value = await store.get("label") as ILabel[];
  label.value.unshift({
    label: "默认",
    value: 0
  });
  label.value.unshift({
    label: "全部",
    value: -1
  });
})
</script>