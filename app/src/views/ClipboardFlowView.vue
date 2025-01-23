<script setup lang="ts">
import { listen, emit } from '@tauri-apps/api/event';
import { onMounted, shallowRef } from 'vue';
import { ClipboardFlowData } from '../types';
import VRunesWithDps from '../components/VDpsWithRunes.vue';
import { fmt } from '../formatter';

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
		<div>
			{{ data.weapon.weapon.base }}
			<span class="pr-2" v-if="data.weapon.weapon.quality">+{{ data.weapon.weapon.quality }}%</span>
			<strong>{{ data.weapon.dps.total }}</strong>
		</div>

		<VRunesWithDps
			:is_winner="true"
			:rune_size="65"
			:show_runes_names="true"
			:runes_with_dps="data.weapon.dps_with_different_runes[0]"
		>
			<template v-slot:right
				><div class="text-green-600 text-3xl pl-2">
					+{{ fmt((data.weapon.dps_with_different_runes[0].dps.total / data.weapon.dps.total) * 100 - 100) }}%
				</div></template
			>
		</VRunesWithDps>

		<details>
			<summary>Other runes</summary>
			<ul>
				<li
					:key="runes_with_dps.runes.join('')"
					v-for="runes_with_dps in data.weapon.dps_with_different_runes.slice(1)"
				>
					<VRunesWithDps :runes_with_dps="runes_with_dps" />
				</li>
			</ul>
		</details>
	</div>
</template>

<style scoped></style>
