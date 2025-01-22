<script setup lang="ts">
import { Rune } from '../types';
const { size = 40, show_runes_names = false } = defineProps<{
	variant: Rune;
	size?: number;
	show_runes_names?: boolean;
}>();
</script>

<template>
	<div class="flex flex-col items-center">
		<img :width="size" :height="size" :src="`/runes/${variant}.webp`" :alt="variant" />
		<div v-if="show_runes_names" class="text-sm font-semibold">{{ variant }}</div>
	</div>
</template>

<style scoped></style>
