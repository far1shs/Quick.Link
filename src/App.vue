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

const router = useRouter();

onMounted(async () => {
  if (await invoke("check_file_exists", {
    path: "./core/frpc.exe"
  }) == false) {
    router.push({
      path: `/download`,
      query: {
        name: "frpc",
        url: "https://file.rivfox.com/frpc-windows.exe",
        save_path: "./core/frpc.exe"
      }
    })
  }
})
</script>