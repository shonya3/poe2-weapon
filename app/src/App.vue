<template>
	<div class="w-full min-h-lvh bg-stone-200 relative">
		<CustomTitleBar />

		<main class="px-4">
			<RouterView />
		</main>

		<LatestReleaseLink class="absolute bottom-0.5 right-2" />
	</div>
</template>

<script setup lang="ts">
import CustomTitleBar from './components/CustomTitleBar.vue';
import LatestReleaseLink from './components/LatestReleaseLink.vue';
</script>
