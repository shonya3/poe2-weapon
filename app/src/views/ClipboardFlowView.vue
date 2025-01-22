<script setup lang="ts">
import { listen, emit } from '@tauri-apps/api/event';
import { onMounted, shallowRef } from 'vue';
import { ClipboardFlowData } from '../types';
import VRunesWithDps from '../components/VRunesWithDps.vue';

const ready = shallowRef(false);
const not_ready_message = shallowRef<null | 'yes'>(null);
const data = shallowRef<ClipboardFlowData | null>(null);

listen<ClipboardFlowData>('clipboard-flow-data', ({ payload }) => {
	ready.value = true;
	data.value = payload;
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

	<div v-if="!data">
		<p>Data is not ready</p>
		<pre>Asked for data resend: {{ not_ready_message ?? 'NOT SENT' }}</pre>
	</div>
	<div v-else>
		{{ data.weapon.base }} <strong>{{ data.dps.total }}</strong>

		<ul>
			<li :key="runes_with_dps.runes.join('')" v-for="runes_with_dps in data.runes">
				<VRunesWithDps :runes_with_dps="runes_with_dps" />
			</li>
		</ul>
	</div>
</template>

<style scoped></style>
