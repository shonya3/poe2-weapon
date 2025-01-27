<script setup lang="ts">
import { fmt } from '../formatter';
import { Dps } from '../types';
import '../elements/simple-tooltip';
defineProps<{ dps: Dps }>();
</script>

<template>
	<div>
		<div>{{ fmt(dps.total) }}</div>
		<simple-tooltip>
			<div class="space-y-2 p-4 bg-white border border-gray-200 rounded-lg shadow-lg text-base">
				<div v-if="dps.pdps" class="text-amber-900">
					⚙️ Physical: <span>{{ fmt(dps.pdps) }}</span>
				</div>
				<div v-if="dps.edps" class="text-sky-600">🌊 Elemental: {{ fmt(dps.edps) }}</div>
				<div v-if="dps.cdps" class="text-purple-800">🌀 Chaos: {{ fmt(dps.cdps) }}</div>
			</div>
		</simple-tooltip>
	</div>
</template>
