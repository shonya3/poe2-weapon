<script setup lang="ts">
import { DpsWithRunes } from '../types';
import Rune from './VRune.vue';
import VDps from './VDps.vue';

const {
	show_runes_names = false,
	rune_size = 40,
	is_winner = false,
} = defineProps<{
	runes_with_dps: DpsWithRunes;
	show_runes_names?: boolean;
	rune_size?: number;
	is_winner?: boolean;
}>();
</script>

<template>
	<div :class="is_winner ? 'p-2' : ''" class="flex items-center gap-2">
		<slot name="left" />
		<div class="flex">
			<Rune :size="rune_size" :show_runes_names="show_runes_names" :variant="runes_with_dps.runes[0]" />
			<Rune
				v-if="runes_with_dps.runes[1]"
				:size="rune_size"
				:show_runes_names="show_runes_names"
				:variant="runes_with_dps.runes[1]"
			/>
		</div>

		<VDps :class="is_winner ? ' text-stone-800 text-4xl' : ''" :dps="runes_with_dps.dps" />
		<slot name="right" />
	</div>
</template>
