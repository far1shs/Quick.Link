<template>
  {{ urlParams }}
</template>

<script setup lang="ts">
import {onMounted} from "vue";
import {AppSideBarSelect} from "@/model.ts";

const urlParams = new URLSearchParams(window.location.search);

onMounted(() => {
  AppSideBarSelect.value = urlParams.get("id") ?? "";
})
</script>