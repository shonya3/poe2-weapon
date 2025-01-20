<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { onMounted, shallowRef } from 'vue';

const ready = shallowRef(false);

onMounted(() => {
	listen('ready', () => {
		ready.value = true;
	});
});
</script>

<template>
	<h2>About</h2>
	Ready? {{ ready }}
</template>

<style scoped></style>
