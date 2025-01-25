<script setup lang="ts">
import { Link } from './Link';

const version = import.meta.env.VERSION;
const tag = `app-v${version}`;
const label = `v${version}`;
const href = `https://github.com/shonya3/poe2-weapon/releases/tag/${tag}`;
</script>

<template>
	<Link kind="browser" :to="href">{{ label }}</Link>
</template>
