<template>
  <Dialog>
    <DialogTrigger>
      <Button variant="ghost" class="w-7 h-7">
        <IconAddRounded/>
      </Button>
    </DialogTrigger>
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>创建标签</DialogTitle>
      </DialogHeader>

      <Input v-model="newLabel" placeholder="输入标签名"/>

      <DialogClose>
        <Button @click="addLabelClick" class="mt-1 w-full">创建</Button>
      </DialogClose>
    </DialogContent>
  </Dialog>

  <DropdownMenu>
    <DropdownMenuTrigger>
      <Button variant="ghost" class="h-7 px-2 text-[13px]">
        <span>{{ type.find((item) => item!.value == typeSelect)?.label }}</span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-48">
      <DropdownMenuItem v-for="item in type"
                        @click="typeSelectClick(item.value)"
                        :inset="typeSelect != item.value">
        <IconCheckRounded v-if="typeSelect == item.value"/>
        {{ item.label }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>

<script setup lang="ts">
import {DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger} from "@/components/ui/dropdown-menu";
import {IconAddRounded, IconCheckRounded} from "@iconify-prerendered/vue-material-symbols";
import {Button} from "@/components/ui/button";
import {onMounted, ref} from "vue";
import {emitter} from "@/lib/eventBus.ts";
import {Dialog, DialogClose, DialogContent, DialogHeader, DialogTitle, DialogTrigger} from "@/components/ui/dialog";
import {Input} from "@/components/ui/input";
import {useRoute} from "vue-router";
import {Store} from "@tauri-apps/plugin-store";
import {ILabel} from "@/type/label.ts";

const type = [
  {
    label: "隧道",
    value: "tunnel"
  }, {
    label: "节点",
    value: "node"
  }
]
const typeSelect = ref<"tunnel" | "node">("tunnel");
const newLabel = ref("");

async function addLabelClick() {
  const store = await Store.load(`${typeSelect.value}.json`, {
    autoSave: true
  });
  const data = await store.get("label") as ILabel[];

  let id = generate8DigitRandomNumber();

  while (true) {
    const dataFind = data.find((item) => item.value == id)
    if (dataFind) id = generate8DigitRandomNumber();
    else break;
  }

  data.push({
    label: newLabel.value,
    value: id
  });

  await store.set("label", data);
  emitter.emit("label_manage_update", {
    type: typeSelect.value
  });
}

function typeSelectClick(value: string) {
  if (value == "tunnel" || value == "node") {
    typeSelect.value = value;
    emitter.emit("label_manage_update", {
      type: typeSelect.value
    });
  }
}

function generate8DigitRandomNumber(): number {
  return Math.floor(10000000 + Math.random() * 90000000);
}

onMounted(async () => {
  const route = useRoute();

  if (route.query.type == "tunnel" || route.query.type == "node") {
    typeSelect.value = route.query.type;
    emitter.emit("label_manage_update", {
      type: typeSelect.value
    });
  }
})
</script>