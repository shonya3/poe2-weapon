<script setup lang="ts">
import { WebviewWindow, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted, shallowRef } from 'vue';

const webview_window = shallowRef<WebviewWindow | null>(null);

onMounted(async () => {
	webview_window.value = getCurrentWebviewWindow();
});
</script>

<template>
	<div data-tauri-drag-region class="flex items-center bg-sky-400 h-[30px] pl-3 sticky top-0">
		<div data-tauri-drag-region class="text-center font-semibold w-full select-none">PoE2 Weapon</div>
		<button
			@click="() => webview_window?.close()"
			class="ml-auto flex justify-center items-center w-[30px] h-[30px] transition-all duration-150 ease-in-out hover:bg-sky-500 hover:shadow-lg"
		>
			<svg aria-label="close" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
				<path
					fill="currentColor"
					d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"
				/>
			</svg>
		</button>
	</div>
</template>
