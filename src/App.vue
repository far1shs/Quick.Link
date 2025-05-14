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
</script>