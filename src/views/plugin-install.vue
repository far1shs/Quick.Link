<template>

</template>

<script setup lang="ts">
import {useRoute} from "vue-router";
import {AppTitle} from "@/model.ts";

const route = useRoute();
const data = JSON.parse(decodeURIComponent(route.query.data as string))

AppTitle.value = `插件安装 - ${data.name}`;
</script>