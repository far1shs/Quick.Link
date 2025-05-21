<template>
  <Dialog v-model:open="open">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>修改隧道</DialogTitle>
      </DialogHeader>

      <Accordion type="single" class="w-full" collapsible default-value="default">
        <AccordionItem value="default">
          <AccordionTrigger>基础</AccordionTrigger>
          <AccordionContent>
            <div class="space-y-4 px-[1px]">
              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="name">名称</Label>
                <Input v-model="tunnelData!.name" id="name" placeholder="名称"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="ip">IP</Label>
                <Input v-model="tunnelData!.ip" id="ip" placeholder="IP"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="port">端口</Label>
                <Input v-model="tunnelData!.port"
                       id="port"
                       placeholder="端口"
                       oninput="this.value = this.value.replace(/[^0-9]/g, '').slice(0, 5)"
                />
              </div>

              <div v-if="tunnelData!.protocol == 'tcp' || tunnelData!.protocol == 'udp'"
                   class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="remote_port">远程端口</Label>
                <Input v-model="tunnelData!.remote_port"
                       id="remote_port"
                       placeholder="远程端口"/>
              </div>

              <div v-else class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="custom_domains">域名绑定组, 空格区分</Label>
                <Input v-model="tunnelData!.domain"
                       id="custom_domains"
                       placeholder="域名绑定组, 空格区分"/>
              </div>
            </div>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="senior">
          <AccordionTrigger>高级</AccordionTrigger>
          <AccordionContent>
            <div class="space-y-4 px-[1px]">
              <div class="grid grid-cols-2 ">
                <div class="grid w-full max-w-sm items-center gap-1.5">
                  <Label for="encrypt">加密</Label>
                  <Switch v-model="tunnelData!.encrypt" id="encrypt"/>
                </div>
                <div class="grid w-full max-w-sm items-center gap-1.5">
                  <Label for="compress">压缩</Label>
                  <Switch v-model="tunnelData!.compress" id="compress"/>
                </div>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="args">其他参数</Label>
                <Input v-model="tunnelData!.args" id="args" placeholder="比如 -xx xxx"/>
              </div>
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>

      <DialogFooter>
        <RouterLink to="/tunnel" class="w-full">
          <Button @click="editTunnelClick" size="sm" class="w-full">
            修改
          </Button>
        </RouterLink>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import {Accordion, AccordionContent, AccordionItem, AccordionTrigger} from "@/components/ui/accordion";
import {Input} from "@/components/ui/input";
import {Label} from "@/components/ui/label";
import {Store} from "@tauri-apps/plugin-store";
import {ITunnel} from "@/type/tunnel.ts";
import {emitter} from "@/lib/eventBus.ts";
import {onMounted, onUnmounted, ref} from "vue";
import {Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle} from "@/components/ui/dialog";
import {Switch} from "@/components/ui/switch";
import {Button} from "@/components/ui/button";
import {toast} from "vue-sonner";

const open = ref(false);
let tunnelData = ref<ITunnel | null>(null);

async function editTunnelClick() {
  const store = await Store.load("tunnel.json", {
    autoSave: true,
  });

  const data = await store.get("tunnel") as ITunnel[];
  const dataIndex = data.findIndex((item) => item.id == tunnelData.value!.id);
  if (dataIndex != -1) {
    data[dataIndex] = tunnelData.value!;
    await store.set("tunnel", data);

    open.value = false;
    emitter.emit("tunnel_update");
    toast.success("修改成功");
  }
}

onMounted(() => {
  emitter.on("tunnel_edit", (value) => {
    open.value = true;
    tunnelData.value = value;
  });
});

onUnmounted(() => emitter.off("tunnel_edit"));
</script>