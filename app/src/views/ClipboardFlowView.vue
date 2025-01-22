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
		{{ data.weapon.weapon.base }} <strong>{{ data.weapon.dps.total }}</strong>

		<ul>
			<li
				:key="runes_with_dps.runes.join('')"
				v-for="(runes_with_dps, index) in data.weapon.dps_with_different_runes"
			>
				<VRunesWithDps
					:is_winner="index === 0"
					:rune_size="index === 0 ? 65 : 40"
					:show_runes_names="index === 0"
					:runes_with_dps="runes_with_dps"
				>
					<template v-if="index === 0" v-slot:right
						><div class="text-green-600 text-3xl pl-2">
							+{{ fmt((runes_with_dps.dps.total / data.weapon.dps.total) * 100 - 100) }}%
						</div></template
					>
				</VRunesWithDps>
			</li>
		</ul>
	</div>
</template>

<style scoped></style>
