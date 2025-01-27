<template>
	<div class="w-full min-h-lvh bg-stone-50 relative">
		<CustomTitleBar />

		<main class="px-2">
			<RouterView />
		</main>

		<UpdateAppLink class="absolute bottom-0.5 left-2 underline" />
		<LatestReleaseLink class="absolute bottom-0.5 right-2" />
	</div>
</template>

<script setup lang="ts">
import CustomTitleBar from './components/CustomTitleBar.vue';
import LatestReleaseLink from './components/links/LatestReleaseLink.vue';
import UpdateAppLink from './components/links/UpdateAppLink.vue';
</script>
