<script setup lang="ts">
import { Rune } from '../types';
const { size = 40, show_runes_names = false } = defineProps<{
	variant: Rune;
	size?: number;
	show_runes_names?: boolean;
}>();
</script>

<template>
	<div>
		<div class="flex flex-col items-center">
			<img :width="size" :height="size" :src="`/runes/${variant}.webp`" :alt="variant" />
			<div v-if="show_runes_names" class="text-sm font-semibold">
				{{ variant.replace('Greater', 'G. ').replace('Lesser', 'L. ') }}
			</div>
		</div>
		<simple-tooltip>
			<div class="space-y-2 p-4 bg-white border border-gray-200 rounded-lg shadow-lg text-base">
				{{ variant.replace('Lesser', 'Lesser ').replace('Greater', 'Greater ') }}
			</div>
		</simple-tooltip>
	</div>
</template>
