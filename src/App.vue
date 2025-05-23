<template>
  <div>
    <SidebarProvider :open="false">
      <AppSideBar/>
      <main style="width: 100%">
        <AppHeader/>
        <div style="margin: 0 12px 10px 12px">
          <RouterView/>
        </div>
      </main>
    </SidebarProvider>

    <Toaster style="margin-right: -107px; margin-bottom: -20px" :toastOptions="{style: { width: '270px' },}"/>
  </div>
</template>

<script setup lang="ts">
import {SidebarProvider} from "@/components/ui/sidebar";
import AppSideBar from "@/components/AppSideBar.vue";
import AppHeader from "@/components/AppHeader.vue";
import {Toaster} from "@/components/ui/sonner";
import {invoke} from "@tauri-apps/api/core";
import {onMounted} from "vue";
import {useRouter} from "vue-router";
import {toast} from "vue-sonner";

const router = useRouter();

onMounted(async () => {
  const path = await invoke<string>("get_roaming_dir");

  const core_frpc = await invoke<string>("join_paths", {
    base: await invoke("join_paths", {
      base: path,
      sub: "core"
    }),
    sub: "frpc.exe"
  });

  if (path != "") {
    if (await invoke("check_file_exists", {
      path: core_frpc
    }) == false) {
      router.push({
        path: `/download`,
        query: {
          name: "frpc",
          url: "https://file.rivfox.com/frpc-windows.exe",
          save_path: encodeURIComponent(core_frpc)
        }
      })
    }
  } else {
    toast.error("找不到 Roaming 目录, 无法检测 FRP 可用性");
  }
})
</script>