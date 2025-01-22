<script setup lang="ts">
import { Dps } from '../types';
defineProps<{ dps: Dps }>();
</script>

<template>
	<div>{{ dps.total }}</div>
</template>

<style scoped></style>
