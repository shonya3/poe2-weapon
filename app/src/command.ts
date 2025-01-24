import { invoke } from '@tauri-apps/api/core';

export interface Commands {
	open_browser: (args: { url: string }) => void;
}

const debug = false;
export const command = async <CommandName extends keyof Commands, Fn extends Commands[CommandName]>(
	name: CommandName,
	...args: Parameters<Fn>
): Promise<ReturnType<Fn>> => {
	if (debug) {
		const t0 = performance.now();
		const res = (await invoke(name, ...args)) as ReturnType<Fn>;
		console.log(`${name}: ${new Intl.NumberFormat().format(performance.now() - t0)}ms`);
		return res;
	} else return invoke(name, ...args) as Promise<ReturnType<Fn>>;
};
