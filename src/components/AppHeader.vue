<template>
  <div class="h-11.5 flex items-center justify-between p-2" data-tauri-drag-region>
    <div>
      <span class="truncate font-semibold cursor-default text-[14px]"
            style="margin-left: 4px"
            data-tauri-drag-region>
        {{ AppTitle }}
      </span>
    </div>

    <div class="flex items-center gap-1">
      <component :is="AppTitleAction"/>

      <div v-if="AppTitleAction" class="bg-[var(--border)] h-4 w-[1px] mx-1"/>

      <Button @click="appWindow.minimize()" variant="ghost" class="w-7 h-7">
        <IconMinimizeRounded style="margin-top: -11px"/>
      </Button>
      <Button @click="appWindow.hide()" variant="ghost" class="w-7 h-7">
        <IconCloseRounded/>
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {getCurrentWindow} from "@tauri-apps/api/window";
import {
  IconMinimizeRounded,
  IconCloseRounded,
} from "@iconify-prerendered/vue-material-symbols";
import {Button} from "@/components/ui/button";
import {AppTitle, AppTitleAction} from "../model.ts";

const appWindow = getCurrentWindow();
</script>

<style scoped>

</style>