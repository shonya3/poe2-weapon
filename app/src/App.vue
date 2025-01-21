<template>
	<CustomTitleBar />
	<nav>
		<RouterLink to="/">Home </RouterLink>
		<RouterLink to="/clipboard-flow">Clipboard Flow</RouterLink>
	</nav>
	<main>
		<RouterView />
	</main>
</template>

<script setup lang="ts">
import CustomTitleBar from './CustomTitleBar.vue';
</script>
