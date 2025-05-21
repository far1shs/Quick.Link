<template>
  <Sidebar v-if="AppSideBarShow" collapsible="icon">
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in items">
              <SidebarMenuButton @click="itemSelected(item.value)"
                                 :tooltip="item.label"
                                 asChild
                                 class="transition-colors duration-250"
                                 :isActive="AppSideBarSelect == item.value">
                <router-link :to="item.url">
                  <component :is="item.icon"/>
                  <span>{{ item.label }}</span>
                </router-link>
              </SidebarMenuButton>
            </SidebarMenuItem>

            <div v-if="pluginItems?.length == 0"
                 class="separator"/>

            <SidebarMenuItem v-for="item in pluginItems">
              <SidebarMenuButton @click="itemSelected(item.id.toString())"
                                 :tooltip="item.label"
                                 asChild
                                 class="transition-colors duration-250"
                                 :isActive="AppSideBarSelect == item.id.toString()">
                <router-link :to="`/plugin-view?id=${item.id}`">
                  <component :is="item.icon"/>
                  <span>{{ item.label }}</span>
                </router-link>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem key="user">
          <DropdownMenu>
            <DropdownMenuTrigger as-child>
              <SidebarMenuButton
                  size="lg"
                  class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
              >
                <Avatar class="h-8 w-8 rounded-lg">
                  <AvatarImage :src="userInfo.avatar" :alt="userInfo.name"/>
                  <AvatarFallback class="rounded-lg">
                    <IconPersonOutlineRounded/>
                  </AvatarFallback>
                </Avatar>

                <div class="grid flex-1 text-left text-sm leading-tight">
                  <span class="truncate font-semibold">{{ userInfo.name }}</span>
                  <span class="truncate text-xs">{{ userInfo.email }}</span>
                </div>
              </SidebarMenuButton>
            </DropdownMenuTrigger>
            <DropdownMenuContent
                class="w-[--reka-dropdown-menu-trigger-width] min-w-56 rounded-lg"
                side="right"
                align="end"
                :side-offset="4"
            >
              <DropdownMenuLabel class="p-0 font-normal">
                <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                  <Avatar class="h-8 w-8 rounded-lg">
                    <AvatarImage :src="userInfo.avatar" :alt="userInfo.name"/>
                    <AvatarFallback class="rounded-lg">
                      <IconPersonOutlineRounded/>
                    </AvatarFallback>
                  </Avatar>

                  <div class="grid flex-1 text-left text-sm leading-tight">
                    <span class="truncate font-semibold">{{ userInfo.name }}</span>
                    <span class="truncate text-xs">{{ userInfo.email }}</span>
                  </div>
                </div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator/>
              <DropdownMenuGroup>
                <RouterLink to="/settings">
                  <DropdownMenuItem>
                    <IconSettingsOutlineRounded/>
                    设置
                  </DropdownMenuItem>
                </RouterLink>
                <RouterLink to="/label-manage">
                  <DropdownMenuItem>
                    <IconSellOutline/>
                    标签管理
                  </DropdownMenuItem>
                </RouterLink>
                <DropdownMenuItem disabled>
                  <IconLogoutRounded/>
                  退出登录
                </DropdownMenuItem>
              </DropdownMenuGroup>
            </DropdownMenuContent>
          </DropdownMenu>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>
  </Sidebar>
</template>

<script setup lang="ts">
import {
  IconDnsOutline,
  IconFormatListBulletedRounded,
  IconExtensionOutline,
  IconPersonOutlineRounded,
  IconSettingsOutlineRounded,
  IconLogoutRounded,
  IconSellOutline
} from "@iconify-prerendered/vue-material-symbols";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {Avatar, AvatarImage, AvatarFallback} from "@/components/ui/avatar";
import {AppSideBarSelect, AppSideBarShow} from "@/model.ts";
import {ref} from "vue";
import {IPluginItem} from "@/type/plugin.ts";

const userInfo = {
  name: "没做",
  email: "",
  avatar: "",
};
const items = [
  {
    label: "隧道",
    value: "tunnel",
    icon: IconFormatListBulletedRounded,
    url: "/tunnel",
  }, {
    label: "节点",
    value: "node",
    icon: IconDnsOutline,
    url: "/node",
  }, {
    label: "扩展",
    value: "plugin",
    icon: IconExtensionOutline,
    url: "/plugin",
  },
];
const pluginItems = ref<IPluginItem[] | null>(null);

function itemSelected(id: string) {
  AppSideBarSelect.value = id;
}
</script>