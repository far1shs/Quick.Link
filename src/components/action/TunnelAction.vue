<template>
  <RouterLink to="/tunnel-add">
    <Button variant="ghost" class="w-7 h-7">
      <IconAddRounded/>
    </Button>
  </RouterLink>

  <DropdownMenu>
    <DropdownMenuTrigger>
      <Button variant="ghost" class="h-7 px-2 text-[13px]">
        <span>{{ label?.find((item) => item!.value == labelSelect)?.label }}</span>
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

  <TcpUdpTunnel/>
</template>

<script setup lang="ts">
import {
  IconAddRounded,
  IconCheckRounded,
} from "@iconify-prerendered/vue-material-symbols";
import {Button} from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {onMounted, ref} from "vue";
import {Store} from "@tauri-apps/plugin-store";
import {emitter} from "@/lib/eventBus.ts";
import TcpUdpTunnel from "@/components/action/tunnel/TcpUdpTunnel.vue";
import {ILabel} from "@/type/label.ts";

const label = ref<ILabel[] | null>(null);
const labelSelect = ref(-1);

function labelSelectClick(id: number) {
  labelSelect.value = id;
  emitter.emit("tunnel_update", {
    label: labelSelect.value,
  });
}

onMounted(async () => {
  const store = await Store.load("tunnel.json");
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