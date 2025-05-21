<template>
  <Dialog v-model:open="open">
    <DialogContent class="w-100 p-4">
      <DialogHeader>
        <DialogTitle>创建隧道</DialogTitle>
      </DialogHeader>

      <Accordion type="single" class="w-full" collapsible default-value="default">
        <AccordionItem value="default">
          <AccordionTrigger>基础</AccordionTrigger>
          <AccordionContent>
            <div class="space-y-4 px-[1px]">
              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="name">名称</Label>
                <Input v-model="newTunnelData.name" id="name" placeholder="名称"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="ip">IP</Label>
                <Input v-model="newTunnelData.ip" id="ip" placeholder="IP"/>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="port">端口</Label>
                <Input v-model="newTunnelData.port"
                       id="port"
                       placeholder="端口"
                       oninput="this.value = this.value.replace(/[^0-9]/g, '').slice(0, 5)"
                />
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="remote_port">远程端口</Label>
                <Input v-model="newTunnelData.remote_port"
                       id="remote_port"
                       placeholder="远程端口"
                       oninput="this.value = this.value.replace(/[^0-9]/g, '').slice(0, 5)"/>
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
                  <Switch v-model="newTunnelData.encrypt" id="encrypt"/>
                </div>
                <div class="grid w-full max-w-sm items-center gap-1.5">
                  <Label for="compress">压缩</Label>
                  <Switch v-model="newTunnelData.compress" id="compress"/>
                </div>
              </div>

              <div class="grid w-full max-w-sm items-center gap-1.5">
                <Label for="args">其他参数</Label>
                <Input v-model="newTunnelData.args" id="args" placeholder="比如 -xx xxx"/>
              </div>
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>

      <DialogFooter>
        <RouterLink to="/tunnel" class="w-full">
          <Button @click="newTunnelClick" size="sm" class="w-full">
            创建
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
import {onMounted, onUnmounted, reactive, ref, watch} from "vue";
import {Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle} from "@/components/ui/dialog";
import {Switch} from "@/components/ui/switch";
import {Button} from "@/components/ui/button";
import {toast} from "vue-sonner";

const open = ref(false);
let newTunnelData = reactive<ITunnel>({
  id: generate8DigitRandomNumber(),
  name: `QLT-${generate8DigitRandomNumber()}`,
  label: 0,
  protocol: "tcp",
  ip: "127.0.0.1",
  port: undefined,
  remote_port: generate1000_65535DigitRandomNumber(),
  domain: undefined,
  node_id: 0,
  node_ip: "",
  node_name: "",
  node_port: 0,
  encrypt: false,
  compress: false,
  args: "",
  status: false
});

async function newTunnelClick() {
  const store = await Store.load("tunnel.json", {
    autoSave: true,
  });

  const data = await store.get("tunnel") as ITunnel[];

  while (true) {
    const dataFind = data.find((item) => item.id == newTunnelData.id)
    if (dataFind) newTunnelData.id = generate8DigitRandomNumber();
    else break;
  }

  data.push(newTunnelData);

  await store.set("tunnel", data);
  open.value = false;
  emitter.emit("tunnel_update");
  toast.success("创建成功");
}

function generate8DigitRandomNumber(): number {
  return Math.floor(10000000 + Math.random() * 90000000);
}

function generate1000_65535DigitRandomNumber(): number {
  return Math.floor(1000 + Math.random() * (65535 - 1000 + 1));
}

onMounted(() => {
  emitter.on("tunnel_add_tcp_udp", (value) => {
    open.value = true;
    newTunnelData = {
      ...newTunnelData,
      protocol: value.protocol,
      node_id: value.node_id,
      node_ip: value.node_ip,
      node_name: value.node_name,
      node_port: value.node_port,
    };
  });

  watch(() => open.value, () => {
    // 重置数据
    if (!open.value) {
      newTunnelData = reactive<ITunnel>({
        id: generate8DigitRandomNumber(),
        name: `QLT-${generate8DigitRandomNumber()}`,
        label: 0,
        protocol: "tcp",
        ip: "127.0.0.1",
        port: undefined,
        remote_port: generate1000_65535DigitRandomNumber(),
        domain: undefined,
        node_id: 0,
        node_ip: "",
        node_name: "",
        node_port: 0,
        encrypt: false,
        compress: false,
        args: "",
        status: false
      });
    }
  });
});

onUnmounted(() => emitter.off("tunnel_add_tcp_udp"));
</script>