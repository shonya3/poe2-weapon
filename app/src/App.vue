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
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { onMounted, ref, shallowRef } from 'vue';
const LABEL = 'TheUniqueLabel';
const webview = shallowRef<WebviewWindow | null>(null);

const counter = ref(0);

function inc() {
	counter.value += 1;
	console.log(counter.value);
}

async function close_secondary_window() {
	const window = await WebviewWindow.getByLabel(LABEL);
	console.log(`Secondary window: `, window);
	await window?.close();
}

// window.addEventListener('keydown', async e => {
// 	if (!(e.ctrlKey && e.key === 'c')) return;

// 	try {
// 		if (!(await custom_window_exists())) {
// 			await create_webview();
// 			await webview.value?.show();
// 		}
// 	} catch (err) {
// 		console.log(err);
// 	}
// });

// async function custom_window_exists(): Promise<boolean> {
// 	return Boolean(await WebviewWindow.getByLabel(LABEL));
// }

// async function create_webview() {
// 	try {
// 		if (!(await WebviewWindow.getByLabel(LABEL))) {
// 			webview.value = new WebviewWindow(LABEL, {
// 				url: '/about',
// 				width: 700,
// 				height: 700,
// 				x: 0,
// 				y: 0,
// 				alwaysOnTop: true,
// 				visible: true,
// 				decorations: false,
// 			});
// 			webview.value.once('tauri://created', function (e) {
// 				// webview successfully created
// 				// webview.value!.hide();
// 				console.log('Webview created', e);
// 			});
// 			webview.value.once('tauri://error', function (e) {
// 				// an error happened creating the webview
// 				console.log(e);
// 			});
// 		}
// 	} catch (err) {
// 		console.log(err);
// 	}
// }

onMounted(async () => {
	// create_webview();
});
</script>
