<script setup lang="ts">
import { useRouter } from 'vue-router';
import { command } from '../command';
import { use_tauri_updater } from '../composables/use_tauri_updater';
import { onMounted } from 'vue';
defineEmits<{
	'update-clicked': [];
}>();

const LATEST_RELEASE = 'https://github.com/shonya3/poe2-weapon/releases/latest';
const { install_and_relaunch, update } = use_tauri_updater();

onMounted(() => {
	if (!update.value) {
		useRouter().back();
	}
});
</script>

<template>
	<button class="underline font-medium text-blue-600 dark:text-blue-500" @click="$router.back">Back</button>
	<div class="changelog pt-40">
		<h1
			class="mb-4 text-4xl text-center font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white"
		>
			PoE2 Weapon v{{ $route.params.new_version }}
		</h1>
		<a
			class="text-center block m-auto underline font-medium text-blue-600 dark:text-blue-500"
			@click.prevent="command('open_browser', { url: LATEST_RELEASE })"
			:href="LATEST_RELEASE"
			>Check release notes</a
		>

		<button
			@click="install_and_relaunch"
			type="button"
			class="mt-4 text-white bg-gradient-to-br from-purple-600 to-blue-500 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center m-auto block"
		>
			Update
		</button>
	</div>
</template>
