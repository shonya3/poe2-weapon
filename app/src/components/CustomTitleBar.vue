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
			<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
		</button>
	</div>
</template>
