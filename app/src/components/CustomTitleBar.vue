<script setup lang="ts">
import { WebviewWindow, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted, shallowRef } from 'vue';

const webview_window = shallowRef<WebviewWindow | null>(null);

onMounted(async () => {
	webview_window.value = getCurrentWebviewWindow();
});
</script>

<template>
	<div data-tauri-drag-region class="titlebar">
		<div @click="() => webview_window?.close()" class="titlebar-button">
			<img src="https://api.iconify.design/mdi:close.svg" alt="close" />
		</div>
	</div>
</template>

<style scoped>
.titlebar {
	height: 30px;
	/* background: #329ea3; */
	background-color: transparent;
	user-select: none;
	display: flex;
	justify-content: flex-end;
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
	background: #5bbec3;
}
</style>
