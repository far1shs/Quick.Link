<template>
  <div v-if="plugin.loading"
       style="height: calc(100vh - 80px)"
       class="flex items-center justify-center">
    <loading/>
  </div>

  <div v-else-if="plugin.error.show"
       style="height: calc(100vh - 80px)"
       class="flex items-center justify-center flex-col gap-1.5">
    <span class="text-[14px]">发生错误</span>
    <ContextMenu>
      <ContextMenuTrigger>
        <span class="text-xs opacity-40">{{ plugin.error.message }}</span>
      </ContextMenuTrigger>
      <ContextMenuContent class="w-52">
        <ContextMenuItem @click="copyClick">
          <IconContentCopyOutlineRounded/>
          复制
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  </div>

  <RouterLink
      v-else
      v-for="item in plugin.list"
      :to="{
        path: `/plugin-install`,
        query: {
          data: encodeURIComponent(JSON.stringify(item))
        }
      }">
    <HoverCard>
      <HoverCardTrigger>
        <div
            class="h-14 rounded-md px-3 py-2 transition-colors duration-250 hover:bg-[var(--sidebar-accent)] flex justify-between items-center group w-full">
          <div class="flex items-center gap-2">
            <Avatar class="h-9 w-9 rounded-lg">
              <AvatarImage :src="item.logo" :alt="item.name"/>
              <AvatarFallback class="rounded-lg">
                <IconExtensionOutline/>
              </AvatarFallback>
            </Avatar>
            <div class="flex flex-col">
              <span class="text-[14px]">{{ item.name }}</span>
              <span class="-mt-1 text-xs opacity-40">{{ item.version }}</span>
            </div>
          </div>

          <div class="flex flex-col">
            <span class="text-xs opacity-40">{{ item.author }}</span>
            <span class="text-xs opacity-40 text-right">{{ item.type }}</span>
          </div>
        </div>
      </HoverCardTrigger>
      <HoverCardContent align="end" class="p-2.5 text-[14px]">
        <span>发布时间: {{ item.publish_time }}</span>
        <span class="flex items-center">已经过域验证<IconVerifiedOutlineRounded class="ml-1"/>
          {{ item.auth.doname }}
        </span>
      </HoverCardContent>
    </HoverCard>
  </RouterLink>

  <Dialog v-model:open="userNotice">
    <DialogContent>
      <DialogHeader>
        <DialogTitle>用户须知</DialogTitle>
        <DialogDescription>
          <div>
            <span>插件验证仅仅只是域名验证, 并不能完全代表安全, 插件优先使用带有标志认证, 以及开源的插件</span>
            <span class="flex items-center"><IconVerifiedOutlineRounded/> 代表已经过域验证</span>
          </div>
        </DialogDescription>
      </DialogHeader>

      <DialogFooter>
        <Button @click="userNoticeClick" variant="ghost">我知道了</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import {
  IconContentCopyOutlineRounded,
  IconExtensionOutline,
  IconVerifiedOutlineRounded
} from "@iconify-prerendered/vue-material-symbols";
import {onMounted, ref, watch} from "vue";
import {fetch} from "@tauri-apps/plugin-http";
import Loading from "@/components/ui/Loading.vue";
import {ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger} from "@/components/ui/context-menu";
import {Avatar, AvatarFallback, AvatarImage} from "@/components/ui/avatar";
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {Store} from "@tauri-apps/plugin-store";
import {useRouter} from "vue-router";
import {writeText} from "@tauri-apps/plugin-clipboard-manager";
import {toast} from "vue-sonner";

const router = useRouter();
const userNotice = ref(false);

const plugin = ref<{ loading: boolean, error: { show: boolean, message: string }, list: any }>({
  loading: false,
  error: {
    show: false,
    message: "",
  },
  list: []
});

async function getPluginList() {
  plugin.value = {
    loading: true,
    error: {
      show: false,
      message: "",
    },
    list: []
  };

  const url = "https://www.far1sh.icu/api/quick_link_plugin.json";
  const res = await fetch(url, {
    method: "GET",
  });

  if (!res.ok) {
    plugin.value.error = {
      show: true,
      message: res.statusText,
    };
    return;
  }

  res.json()
      .then((res) => {
        plugin.value.list = res.results;
      })
      .catch((err) => {
        plugin.value.error = {
          show: true,
          message: err.message,
        };
      })
      .finally(() => plugin.value.loading = false);
}

async function userNoticeClick() {
  const store = await Store.load("settings.json");
  await store.set("user_notice", true);
  await store.save();

  userNotice.value = false;
}

async function copyClick() {
  await writeText(plugin.value.error.message);
  toast.success("复制成功")
}

onMounted(async () => {
  const store = await Store.load("settings.json");
  if (!await store.get("user_notice") as boolean) userNotice.value = true;

  watch(() => userNotice.value,  async() => {
    if (!await store.get("user_notice") as boolean) router.back();
  })
  await getPluginList();
})
</script>