<template>
	<div class="w-full min-h-lvh bg-stone-200">
		<CustomTitleBar />

		<main class="px-4">
			<RouterView />
		</main>
	</div>
</template>

<script setup lang="ts">
import CustomTitleBar from './components/CustomTitleBar.vue';
</script>
