<script setup lang="ts">
import { Rune } from '../types';
defineProps<{
	variant: Rune;
}>();
</script>

<template>
	<img :src="`/runes/${variant}.webp`" :alt="variant" />
</template>

<style scoped></style>
