<script setup lang="ts">
import BrowserLink from './BrowserLink.vue';

const version = import.meta.env.VERSION;
const tag = `v${version}`;
const href = `https://github.com/shonya3/poe2-version/releases/tag/${tag}`;
</script>

<template>
	<BrowserLink :href="href">{{ tag }}</BrowserLink>
</template>
