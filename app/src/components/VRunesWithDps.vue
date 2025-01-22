<script setup lang="ts">
import { RunesWithDps } from '../types';
import Rune from './VRune.vue';
import VDps from './VDps.vue';

defineProps<{
	runes_with_dps: RunesWithDps;
}>();
</script>

<template>
	<div>
		<div>
			<Rune :variant="runes_with_dps.runes[0]" />
			<Rune :variant="runes_with_dps.runes[1]" />
		</div>

		<VDps :dps="runes_with_dps.dps" />
	</div>
</template>

<style scoped></style>
