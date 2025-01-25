<template>
	<div class="w-full min-h-lvh bg-stone-200 relative">
		<CustomTitleBar />

		<main class="px-2 cf:px-4">
			<RouterView />
		</main>

		<UpdateAppLink class="absolute bottom-0.5 left-2 underline" />
		<LatestReleaseLink class="absolute bottom-0.5 right-2" />
	</div>
</template>

<script setup lang="ts">
import CustomTitleBar from './components/CustomTitleBar.vue';
import LatestReleaseLink from './components/LatestReleaseLink.vue';
import UpdateAppLink from './components/UpdateAppLink.vue';
</script>
