<template>
  <div class="flex flex-col items-center justify-center" style="height: calc(100vh - 80px)">
    <Progress v-model="downloadValue" class="w-[240px]"/>
  </div>
</template>

<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";
import {AppSideBarShow} from "@/model.ts";
import {useRoute, useRouter} from "vue-router";
import {Progress} from "@/components/ui/progress";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
import {toast} from "vue-sonner";

const route = useRoute();
const router = useRouter();
const downloadValue = ref(0);

onMounted(async () => {
  AppSideBarShow.value = false;

  const downloadListen = await listen<number | boolean>(`download_1111`, (event) => {
    if (typeof event.payload === "boolean") {
      downloadListen();
      if (event.payload) {
        toast.success("下载成功");
        router.push("/");
      }
    } else {
      downloadValue.value = event.payload;
    }
  });

  await invoke("download_file", {
    id: 1111,
    url: route.query.url,
    savePath: route.query.save_path
  });
});

onUnmounted(() => AppSideBarShow.value = true);
</script>