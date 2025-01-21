<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { onMounted, shallowRef } from 'vue';

const ready = shallowRef(false);
const payload = shallowRef<object | null>(null);

onMounted(() => {
	listen<object>('clipboard_flow_data', p => {
		ready.value = true;
		payload.value = p;
	});
});
</script>

<template>
	<h2>Overlay</h2>
	Ready? {{ ready }}

	<pre>{{ payload }}</pre>
</template>

<style scoped></style>
