<template>
  <div class="flex flex-col gap-2">
    <div class="border rounded-md h-14 px-3 py-2 flex items-center">
      <IconPaletteOutline class="w-7 h-7"/>
      <div class="flex-1 space-y-1" style="margin-left: 10px">
        <div class="flex flex-col">
          <span style="font-size: 14px">主题</span>
          <span style="margin-top: -4px; font-size: 12px" class="opacity-35">设置应用主题</span>
        </div>
      </div>

      <Select v-model="theme.select">
        <SelectTrigger>
          <SelectValue placeholder="主题" class="w-25"/>
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem v-for="item in theme.options" :value="item.value" @select="themeSelected(item.value)">
              {{ item.label }}
            </SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>

    <div class="border rounded-md h-14 px-3 py-2 flex items-center">
      <IconRocketLaunchOutlineRounded class="w-7 h-7"/>
      <div class="flex-1 space-y-1" style="margin-left: 10px">
        <div class="flex flex-col">
          <span style="font-size: 14px">开机自启</span>
          <span style="margin-top: -4px; font-size: 12px" class="opacity-35">设置应用是否开机时启动</span>
        </div>
      </div>

      <Switch v-model="autoStart" @click="autoStartClick"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  IconPaletteOutline,
  IconRocketLaunchOutlineRounded,
} from "@iconify-prerendered/vue-material-symbols";
import {onMounted, ref} from "vue";
import {AppSideBarSelect, AppTitle} from "@/model.ts";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectGroup,
  SelectItem
} from "@/components/ui/select";
import {useColorMode} from "@vueuse/core";
import {Switch} from "@/components/ui/switch";
import {isEnabled, enable, disable} from "@tauri-apps/plugin-autostart";
import {Store} from "@tauri-apps/plugin-store";

const theme = ref({
  select: "auto",
  options: [
    {
      value: "light",
      label: "浅色"
    },
    {
      value: "dark",
      label: "深色"
    },
    {
      value: "auto",
      label: "系统"
    }
  ],
});
const autoStart = ref(false);

async function themeSelected(value: string) {
  const store = await Store.load("settings.json");
  await store.set("theme", value);
  await store.save();

  useColorMode().value = value as "light" | "dark" | "auto";
}

async function autoStartClick() {
  !autoStart.value ? await enable() : await disable();
}

onMounted(async () => {
  AppSideBarSelect.value = "settings";
  AppTitle.value = "设置";

  const store = await Store.load("settings.json");

  autoStart.value = await isEnabled();
  theme.value.select = await store.get("theme") ? await store.get("theme") as string : "auto";
})
</script>