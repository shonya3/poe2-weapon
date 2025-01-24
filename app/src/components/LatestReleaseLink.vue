<script setup lang="ts">
import BrowserLink from './BrowserLink.vue';

const version = import.meta.env.VERSION;
const tag = `app-v${version}`;
const label = `v${version}`;
const href = `https://github.com/shonya3/poe2-weapon/releases/tag/${tag}`;
</script>

<template>
	<BrowserLink class="underline font-medium text-blue-600 dark:text-blue-500" :href="href">{{ label }}</BrowserLink>
</template>
