import { relaunch } from '@tauri-apps/plugin-process';
import { Update, check } from '@tauri-apps/plugin-updater';
import { shallowRef } from 'vue';

export function use_tauri_updater() {
	const update = shallowRef<Update | null>(null);
	async function check_update() {
		try {
			update.value = await check();
		} catch (err) {
			console.log(`Check update: ${err}`);
		}
	}
	async function install_and_relaunch() {
		try {
			if (!update.value) {
				return;
			}
			await update.value.downloadAndInstall();
			await relaunch();
		} catch (err) {
			console.log(err);
		}
	}

	check_update();

	return {
		update,
		install_and_relaunch,
	};
}
