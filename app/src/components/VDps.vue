<script setup lang="ts">
import { fmt } from '../formatter';
import { Dps } from '../types';
defineProps<{ dps: Dps }>();
</script>

<template>
	<div>{{ fmt(dps.total) }}</div>
</template>

<style scoped></style>
