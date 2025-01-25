<script setup lang="ts">
import { computed } from 'vue';
import { use_tauri_updater } from '../../composables/use_tauri_updater';
import { Link } from './Link';

const { update } = use_tauri_updater();

// const is_visible = computed(() => Boolean(update.value?.available));
// const version = computed(() => update.value?.version);
// const to = computed(() => `/update/${version.value}`);
const is_visible = true;
const version = '0.3.3';
const to = `/update/${version}`;
</script>

<template>
	<Link v-if="is_visible" :to="to" kind="app">Update is Ready</Link>
</template>
