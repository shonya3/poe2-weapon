<template>
	<div class="overflow-auto px-2 relative h-dvh">
		<RouterView />

		<UpdateAppLink class="fixed bottom-0.5 left-2 underline" />
		<LatestReleaseLink class="fixed bottom-0.5 right-4" />
	</div>
</template>

<script setup lang="ts">
import LatestReleaseLink from './components/links/LatestReleaseLink.vue';
import UpdateAppLink from './components/links/UpdateAppLink.vue';
</script>
