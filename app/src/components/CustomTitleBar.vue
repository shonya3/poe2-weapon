<script setup lang="ts">
import { WebviewWindow, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted, shallowRef } from 'vue';

const webview_window = shallowRef<WebviewWindow | null>(null);

onMounted(async () => {
	webview_window.value = getCurrentWebviewWindow();
});
</script>

<template>
	<div data-tauri-drag-region class="titlebar bg-sky-400">
		<div data-tauri-drag-region class="text-center font-semibold w-100">PoE2 Weapon</div>
		<button @click="() => webview_window?.close()" class="titlebar-button ml-auto">
			<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
		</button>
	</div>
</template>

<style scoped>
.titlebar {
	height: 30px;
	/* background: #329ea3; */
	user-select: none;
	align-items: center;
	display: flex;
	position: sticky;
	top: 0;
	left: 0;
	right: 0;
}
.titlebar-button {
	display: inline-flex;
	justify-content: center;
	align-items: center;
	width: 30px;
	height: 30px;
	user-select: none;
	-webkit-user-select: none;
}
.titlebar-button:hover {
	/* background: ; */
}
</style>
