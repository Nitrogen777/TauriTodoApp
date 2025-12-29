<template>
<h2>The fast line price is</h2>
<h1>{{ price }}₪</h1>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';

const price = ref("");

async function fetchPrice() {
    price.value = await invoke("get_price");
}

onMounted(() => fetchPrice())
</script>

<style scoped></style>

<style></style>