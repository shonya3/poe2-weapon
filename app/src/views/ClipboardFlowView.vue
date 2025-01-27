<script setup lang="ts">
import { listen, emit } from '@tauri-apps/api/event';
import { computed, onMounted, ref } from 'vue';
import { ClipboardFlowData, DpsWithRunes } from '../types';
import VRunesWithDps from '../components/VDpsWithRunes.vue';
import VWeapon from '../components/VWeapon.vue';
import { fmt } from '../formatter';

const ready = ref(false);
const not_ready_message = ref<null | 'yes'>(null);
const data = ref<ClipboardFlowData | null>(null);
const apply_quality = ref(true);
const dps_gain_percents = computed(() => {
	if (!data.value || !runes_dps.value[0]) {
		return 0;
	}

	return (runes_dps.value[0].dps.total / data.value.weapon.dps.total) * 100 - 100;
});

const runes_dps = computed<DpsWithRunes[]>(() => {
	if (!data.value) {
		return [];
	}

	if (apply_quality.value && data.value.weapon_q20) {
		return data.value.weapon_q20.dps_with_different_runes;
	}

	return data.value.weapon.dps_with_different_runes;
});

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
	<div v-if="!data">
		<!-- <p>Data is not ready</p>
		<pre>Asked for data resend: {{ not_ready_message ?? 'NOT SENT' }}</pre> -->
		Loading...
	</div>
	<div v-else class="px-2">
		<VWeapon :img="data.img" :weapon="data.weapon.weapon" :dps="data.weapon.dps" />

		<div v-if="data.weapon_q20 && data.weapon.weapon.quality < 20" class="place-items-end ml-auto">
			<div class="flex items-center gap-1 text-xs text-stone-600">
				<label for="apply-quality">Apply 20% quality</label>
				<input
					class="accent-stone-600"
					@change="e => apply_quality =  (e.target as HTMLInputElement).checked"
					:checked="apply_quality"
					type="checkbox"
					id="apply-quality"
				/>
			</div>
		</div>
		<VRunesWithDps :is_winner="true" :rune_size="65" :show_runes_names="true" :runes_with_dps="runes_dps[0]">
			<template v-if="dps_gain_percents > 0" v-slot:right
				><div class="text-emerald-600 text-3xl pl-1">
					+{{ fmt((runes_dps[0].dps.total / data.weapon.dps.total) * 100 - 100) }}%
				</div></template
			>
		</VRunesWithDps>

		<details class="text-stone-600 pt-8">
			<summary>Other runes</summary>
			<ul class="flex flex-wrap gap-x-10 pt-3">
				<li
					class="basis-[120px]"
					:key="runes_with_dps.runes.join('')"
					v-for="runes_with_dps in runes_dps.slice(1)"
				>
					<VRunesWithDps :runes_with_dps="runes_with_dps" />
				</li>
			</ul>
		</details>
	</div>
</template>
