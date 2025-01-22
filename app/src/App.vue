<template>
	<CustomTitleBar />

	<main>
		<RouterView />
	</main>
</template>

<script setup lang="ts">
import CustomTitleBar from './components/CustomTitleBar.vue';
</script>
