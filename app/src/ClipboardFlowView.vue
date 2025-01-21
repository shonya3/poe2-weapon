<script setup lang="ts">
import { listen, emit } from '@tauri-apps/api/event';
import { onMounted, shallowRef } from 'vue';

const ready = shallowRef(false);
const not_ready_message = shallowRef<null | 'yes'>(null);
const payload = shallowRef<object | null>(null);

listen<object>('clipboard-flow-data', p => {
	ready.value = true;
	payload.value = p;
});

onMounted(() => {
	setTimeout(() => {
		if (!ready.value) {
			not_ready_message.value = 'yes';
			emit('clipboard-flow-ask-resend');
		}
	}, 100);
});
</script>

<template>
	<h2>Overlay</h2>
	Ready? {{ ready }}

	<pre>{{ payload }}</pre>

	<pre>Asked for data resend: {{ not_ready_message ?? 'NOT SENT' }}</pre>
</template>

<style scoped></style>
