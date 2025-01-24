<script setup lang="ts">
import { computed } from 'vue';
import { use_tauri_updater } from '../composables/use_tauri_updater';

const { update } = use_tauri_updater();

const is_visible = computed(() => Boolean(update.value?.available));
const version = computed(() => update.value?.version);
const to = computed(() => `/update/${version.value}`);
// const is_visible = true;
// const version = '0.1.0';
// const to = `/update/${version}`;
</script>

<template>
	<RouterLink v-if="is_visible" :to="to" class="underline font-medium text-blue-600 dark:text-blue-500">
		Update is ready
	</RouterLink>
</template>
