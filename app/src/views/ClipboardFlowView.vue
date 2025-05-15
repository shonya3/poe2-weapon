<script setup lang="ts">
import { listen, emit } from '@tauri-apps/api/event';
import { computed, ref } from 'vue';
import { ClipboardFlowData, DpsWithRunes, Rune, RUNE_TIERS, RuneTier } from '../types';
import VRunesWithDps from '../components/VDpsWithRunes.vue';
import VWeapon from '../components/VWeapon.vue';
import { fmt } from '../formatter';
import { useStorage } from '@vueuse/core';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

const data = ref<ClipboardFlowData | null>(null);
const ready = computed(() => data.value !== null);
const apply_quality = ref(true);
const dps_gain_percents = computed(() => {
	if (!data.value || !runes_dps.value[0]) {
		return 0;
	}

	return (runes_dps.value[0].dps.total / data.value.weapon.dps.total) * 100 - 100;
});

const ALL_RUNES_SET = new Set(Array.from(RUNE_TIERS));
const included_tiers = useStorage<Set<RuneTier>>('included_rune_tiers', new Set(Array.from(RUNE_TIERS)));
const excluded_tiers = computed(() => ALL_RUNES_SET.difference(included_tiers.value));

/**  Get tier of rune. */
function get_tier(rune: Rune): RuneTier {
	const lowercased = rune.toLowerCase();

	if (lowercased.includes('lesser')) {
		return 'lesser';
	}

	if (lowercased.includes('greater')) {
		return 'greater';
	}

	return 'normal';
}

const should_render_other_runes = ref(false);
const runes_dps = computed<DpsWithRunes[]>(() => {
	if (!data.value) {
		return [];
	}

	const filter_exluded_tiers = ({ runes }: { runes: [Rune] | [Rune, Rune] }) => {
		for (const rune of runes) {
			const rune_tier = get_tier(rune);
			if (excluded_tiers.value.has(rune_tier)) {
				return false;
			}
		}

		return true;
	};

	if (apply_quality.value && data.value.weapon_q20) {
		return data.value.weapon_q20.dps_with_different_runes.filter(filter_exluded_tiers);
	}

	return data.value.weapon.dps_with_different_runes.filter(filter_exluded_tiers);
});

let logging_on = true;
const start = new Date();
const log = (...args: any[]) => {
	if (!logging_on) {
		return;
	}
	console.log(`[${Date.now() - start.getTime()}ms]`.padEnd(10), ...args);
};

listen<ClipboardFlowData>('clipboard-flow-data', ({ payload }) => {
	data.value = payload;
	log('Event:clipboard-flow-data');
});

window.addEventListener('keydown', (e: KeyboardEvent) => {
	if (e.code === 'Escape') {
		WebviewWindow.getCurrent().close();
	}
});

// Ask to resend data if needed
let asked = false;
const ask_resend_data = () => {
	log('interval');
	if (ready.value) {
		clearInterval(interval_resend);
		log('Clear interval_resend');
	} else if (!asked) {
		asked = true;
		emit('clipboard-flow-ask-resend');
		log('Emit:clipboard-flow-ask-resend');
	}
};
ask_resend_data();
const interval_resend = setInterval(ask_resend_data, 1);
</script>

<template>
	<div v-if="!data">
		<!--  <p>Data is not ready</p> -->
		<!-- Loading... -->
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
		<VRunesWithDps
			v-if="runes_dps[0]"
			:is_winner="true"
			:rune_size="65"
			:show_runes_names="true"
			:runes_with_dps="runes_dps[0]"
		>
			<template v-if="dps_gain_percents > 0" v-slot:right
				><div class="text-emerald-600 text-3xl pl-1">
					+{{ fmt((runes_dps[0].dps.total / data.weapon.dps.total) * 100 - 100) }}%
				</div></template
			>
			<template v-else-if="dps_gain_percents < 0" v-slot:right
				><div class="text-red-600 text-3xl pl-1">
					{{ fmt((runes_dps[0].dps.total / data.weapon.dps.total) * 100 - 100) }}%
				</div></template
			>
		</VRunesWithDps>

		<div class="mt-4">
			<div class="flex items-center gap-4">
				<h3 class="text-lg font-semibold text-gray-800">Rune Tiers</h3>
				<div class="flex flex-wrap gap-3">
					<div v-for="tier in RUNE_TIERS" :key="tier" class="flex items-center">
						<input
							:id="`tier-${tier}`"
							type="checkbox"
							:value="tier"
							v-model="included_tiers"
							class="h-4 w-4 rounded"
						/>
						<label
							:for="`tier-${tier}`"
							class="ml-2 text-sm font-medium text-gray-700 hover:text-gray-900 cursor-pointer"
						>
							{{ tier }}
						</label>
					</div>
				</div>
			</div>
		</div>

		<details v-if="runes_dps.length > 1" class="text-stone-600 pt-8">
			<summary @click="should_render_other_runes = true">Other runes</summary>
			<ul v-if="should_render_other_runes" class="flex flex-wrap gap-x-10 pt-3">
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
