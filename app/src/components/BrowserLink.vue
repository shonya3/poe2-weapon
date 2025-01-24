<script setup lang="ts">
import { command } from '../command';

defineProps<{ href: string }>();
</script>

<template>
	<a @click.prevent="command('open_browser', { url: href })" :href="href" class="underline">
		<slot></slot>
	</a>
</template>
